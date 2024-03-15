use std::{fmt::{self, Display}, iter::Sum, ops::{Add, Div, Mul, Neg}};

use crate::field::Field;

use super::multiindex::MultiIndex;

#[derive(Clone, PartialEq, Eq,Default,Debug)]
pub struct Terms<F:Field>{
    pub coeff:F,
    pub multi_index:MultiIndex
}
impl <F:Field> Ord for Terms<F> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
       /*  if self.multi_index==rhs.multi_index{
            self.coeff.cmp(&rhs.coeff)
        } else { */
        self.multi_index.cmp(&rhs.multi_index)
    }   
    
}
impl <F:Field>PartialOrd for Terms<F> {
    fn partial_cmp(&self,rhs:&Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
    
}
impl <F:Field>Display for Terms<F>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{}", Terms::print_terms(self))
    }
}
impl <F:Field>Terms<F> {
    pub fn is_constant(&self)->bool{
        self.multi_index.is_zero()
    }
    pub fn total_degree(&self)->usize{
        self.multi_index.weight()
    }
    pub fn zero(&self)->Self{
        Terms::new(self.coeff.zero(), MultiIndex::zero())
    }
    pub fn is_zero(&self)->bool{
        &self.zero()==self||(self.is_constant()&&self.coeff.is_zero())||self.coeff.is_zero()
    }
    pub fn new(coeff:F,multi_index:MultiIndex)->Self{
        if multi_index.is_zero(){
            return Terms{coeff,multi_index};
        }
        
        let mut new_multi_index:Vec<usize>=multi_index.0.into_iter().rev().skip_while(|&x| x == 0).collect();
        new_multi_index.reverse();
        Terms{coeff,multi_index:MultiIndex::new(&new_multi_index)}

    }
    pub fn is_divisible_by(&mut self,rhs:&mut Self)->bool{

        rhs.multi_index.is_subtractable_by(&mut self.multi_index)

    }
    pub fn number_of_variables(&self)->usize{
        self.multi_index.0.iter().filter(|a|**a!=0usize).count()
    }
    pub fn print_terms(&self)->String{
        let mut s:Vec<String>=Vec::new();
        let mut str:String;
        if self.is_constant()||self.is_zero(){return format!("{}",self.coeff);}
        for i in 0..self.multi_index.len(){
            if self.multi_index.0[i]==0{continue;}
            if self.multi_index.0[i]==1{
                str=format!("x_{}",i);

            }
            else {
                str =format!("x_{}^{}",i,self.multi_index.0[i]);
            }
        s.push(str);
        }
       format!("{}*{}",self.coeff,s.join("*"))
    }
    

    
}
impl <'a, F:Field> Sum for &'a Terms<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.sum()
    }
}

impl<'a, 'b,F:Field> Add<&'b Terms<F>>for &'a Terms<F> {
    type Output = Terms<F>;
    fn add(self, rhs: &'b Terms<F>) -> Self::Output {
        if self.is_zero(){return rhs.clone();}
        if rhs.is_zero(){return self.clone();}
        if self.multi_index==rhs.multi_index {
            let coeff=self.coeff.clone()+rhs.coeff.clone();
            Terms::new(coeff, self.multi_index.clone())
        } else {panic!("Cannot add {} and {}. No similar monomial!",self,rhs)}
        }
    }
    impl <'a,F:Field> Neg for &'a Terms<F> {
        type Output = Terms<F>;
        fn neg(self) -> Self::Output {
            let coeff=self.coeff.clone().neg();
            Terms::new(coeff,self.multi_index.clone())
        }
        
    }
impl<'a, 'b,F:Field> Mul<&'b mut Terms<F>>for &'a mut Terms<F> {
    type Output = Terms<F>;
/// # Example
/// 
/// ```
/// use algebra::intmod::PrimeField;
/// use algebra::field::Field;
/// use num_bigint::BigInt;
/// use algebra::multivariate::terms::Terms;
/// use algebra::multivariate::multiindex::MultiIndex;
/// use algebra::multivariate::multivariatepoly::MultivariatePoly;
/// let z13=PrimeField(BigInt::from(13));
/// let mut term1 = Terms::new(z13.new(BigInt::from(9)),MultiIndex::new(&vec![2,0]));
/// let mut term2=Terms::new(z13.new(BigInt::from(5)),MultiIndex::new(&vec![1,0,1]));
/// let mul = &mut term1*&mut term2;
/// let expected_mul = Terms::new(z13.new(BigInt::from(6)),MultiIndex::new(&vec![3,0,1]));
/// assert_eq!(mul,expected_mul);
///  
/// ```
    fn mul(self:&'a mut Terms<F>, rhs: &'b mut Terms<F>) -> Self::Output {
        let coeff = self.coeff.clone()*rhs.coeff.clone();
        let multi_index=&mut self.multi_index+&mut rhs.multi_index;
        
        Terms::new(coeff,multi_index)
    }
}
impl<'a, 'b,F:Field> Div<&'b mut Terms<F>> for &'a mut Terms<F> {
    type Output = Terms<F>;
/// # Example
/// 
/// ```
/// use algebra::intmod::PrimeField;
/// use algebra::field::Field;
/// use num_bigint::BigInt;
/// use algebra::multivariate::terms::Terms;
/// use algebra::multivariate::multiindex::MultiIndex;
/// use algebra::multivariate::multivariatepoly::MultivariatePoly;
/// let z13=PrimeField(BigInt::from(13));
///     let mut term1 = Terms::new(z13.new(BigInt::from(5)),MultiIndex::new(&vec![2,3,1]));
///     let mut term2 = Terms::new(z13.new(BigInt::from(4)),MultiIndex::new(&vec![2,1]));
///     let division= &mut term1 / &mut term2;
///     let expected_division=Terms::new(z13.new(BigInt::from(11)),MultiIndex::new(&vec![0,2,1]));
/// assert_eq!(division,expected_division);
/// 
/// ```
    fn div(self:&'a mut Terms<F>, rhs: &'b mut Terms<F>) -> Self::Output {    
        if !self.is_divisible_by(rhs) {panic!("Cannot divide!")}
        let coeff=self.coeff.clone()*rhs.coeff.inverse();
        let multi_index=&mut self.multi_index-&mut rhs.multi_index;
        
        Terms::new(coeff,multi_index)
        }
}
