use core::fmt;
use std::{ops::{Add, Sub, Mul, Div, Rem}, cmp::max};
use num_traits::Zero;

use crate::field::Field;
#[derive(Debug, PartialEq, Clone)]
pub struct Poly<T> {
    pub coeffs: Vec<T>,
}
impl <F:Field+std::fmt::Display+std::cmp::PartialOrd>fmt::Display for Poly<F> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{}", Poly::print_poly(self,"x"))
    }
}
/// # Example
/// ```
/// #[macro_use] extern crate algebra;
/// use num_bigint::BigInt;
/// use algebra::poly::Poly;
/// use algebra::intmod::{Mod,PrimeField};
/// use algebra::field::Field;
/// let prime_base=BigInt::from(13);
/// let z13=PrimeField(Some(prime_base));
/// let p1 = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3)));
/// let p2 = Poly::new_from_coeffs(&[z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3))]);
/// assert_eq!(p1, p2);
/// ```
#[macro_export]
macro_rules! poly {

    ($($c:expr),+ $(,)*) => {
        $crate::poly::Poly::new_from_coeffs(&[$($c,)*])
    };
}

impl <F:Field> Poly <F> {
    pub fn new_from_coeffs(coeff:&[F])->Self {
        Self { coeffs: coeff.to_vec() }
    }
    pub fn len(&self)->usize{
        self.coeffs.len()
    }
    pub fn deg(&mut self) -> usize {
        let vec=&mut self.coeffs;
        if vec.is_empty(){return 0;} else {
        while vec.last().unwrap().is_zero() {vec.remove(vec.len()-1);}
        }
        vec.len()-1
    }
    pub fn extend(&self,new_len:usize)->Vec<F> {
        let zero=self.coeffs[0].zero();
        let mut new_coeffs: Vec<F>=Vec::with_capacity(new_len);
        if self.coeffs.len()<new_len {
            for i in 0..self.len(){
                new_coeffs.push(self.coeffs[i].clone())
            }
            for i in self.len()..new_len {
                new_coeffs.push(zero.clone())
            }
        new_coeffs
        }
        else {
            panic!("cant extend the polinomial. Degree is to high!");
        }
        
    } 
    pub fn zero()->Poly<F> {
        Poly::new_from_coeffs(&vec![])
    } 
}
/// # Example
/// ```
/// #[macro_use] extern crate algebra;
/// use num_bigint::BigInt;
/// use algebra::poly::Poly;
/// use algebra::intmod::{Mod,PrimeField};
/// use algebra::field::Field;
/// let prime_base=BigInt::from(13);
/// let z13=PrimeField(Some(prime_base));
/// let p1 = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3)));
/// let p2 = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(2)));
/// let add = poly!(z13.new(BigInt::from(6)), z13.new(BigInt::from(4)),z13.new(BigInt::from(3)));
/// let sub = poly!(z13.new(BigInt::from(9)), z13.new(BigInt::from(0)),z13.new(BigInt::from(3)));
/// let prod = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(12)),z13.new(BigInt::from(6)),z13.new(BigInt::from(6)));
/// assert_eq!(add,&p1+&p2);
/// assert_eq!(sub,&p1-&p2);
/// assert_eq!(prod,&p1*&p2);
/// 
/// ```
impl <'a,'b,F:Field>Add<&'b Poly<F>>for  &'b Poly<F> {
    type Output=Poly<F>;
    fn add(self, rhs: Self) -> Self::Output {
        let max_len=max(self.coeffs.len(), rhs.coeffs.len());
        let mut v1=self.coeffs.clone();
        let mut v2=rhs.coeffs.clone();
        if self.len()<rhs.len(){
            
            v1=self.extend(max_len);
            v2=rhs.coeffs.clone();
        } else if self.len()>rhs.len(){
            v1=rhs.extend(max_len);
            v2=self.coeffs.clone();
        }
        let mut sum=Vec::with_capacity(v1.len());
        for i in 0..v1.len(){
            sum.push(v1[i].clone()+v2[i].clone());
        }
Poly::new_from_coeffs(&sum)
    }
}
impl <'a,'b,F:Field>Sub<&'b Poly<F>>for  &'b Poly<F> {
    type Output=Poly<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        let max_len=max(self.coeffs.len(), rhs.coeffs.len());
        let mut v1=self.coeffs.clone();
        let mut v2=rhs.coeffs.clone();
        if self.len()<rhs.len(){
            v1=self.extend(max_len);
        } else if self.len()>rhs.len(){
            v2=rhs.extend(max_len);
        }
        let mut sum=Vec::with_capacity(v1.len());
        for i in 0..v1.len(){
            sum.push(v1[i].clone()-v2[i].clone());
        }
Poly::new_from_coeffs(&sum)
    }
}
impl <'a,'b,F:Field>Mul<&'b Poly<F>>for  &'b Poly<F>{
    type Output=Poly<F>;
    fn mul(self, rhs: Self) -> Self::Output {
        let zero=self.coeffs[0].clone().zero();
        let new_len=self.coeffs.len()+rhs.coeffs.len()-1;
        let mut prod=vec![zero;new_len];
        for i in 0..self.len(){
            for j in 0..rhs.len(){
                prod[i+j]=prod[i+j].clone()+self.coeffs[i].clone()*rhs.coeffs[j].clone();
            }
        }
Poly::new_from_coeffs(&prod.to_vec())
    }
}
impl <'a,'b,F:Field>Div<&'b Poly<F>>for  &'b Poly<F>{
    type Output=Poly<F>;
    fn div(self, rhs: &'b Poly<F>) -> Self::Output{
        Poly::div_rem(self, rhs)[0].clone()
}
}
impl <'a,'b,F:Field>Rem<&'b Poly<F>>for  &'b Poly<F>{
    type Output=Poly<F>;
    fn rem(self, rhs: &'b Poly<F>) -> Self::Output{
        Poly::div_rem(self, rhs)[1].clone()
}
}
impl <F:Field+std::fmt::Display+std::cmp::PartialOrd>Poly<F> {
    pub fn print_poly(&self,x:&str)->String{
        let zero=self.coeffs[0].clone().zero();
        let mut s: Vec<String> = Vec::new();
        for (i,n) in self.coeffs.iter().enumerate(){
            if n.is_zero(){continue;}
            let term = if i.is_zero(){n.to_string()} else if i==1 {format!("{}*{}", n, x)}
            else{format!("{}*{}^{}", n, x,i)};
            if !s.is_empty()&&(*n)>zero{
                s.push("+".to_string());
            }
            s.push(term);
        }
        s.concat()
    }
    
}

impl <F:Field>Poly<F> {
    pub fn div_rem(g:&Poly<F>,h:&Poly<F>)->Vec<Poly<F>>{
            let zero=g.coeffs[0].clone().zero();
            let mut rem=g.coeffs.clone();
            let new_len=g.len()-h.len()+1;
            let mut q: Vec<F>=vec![zero;new_len];
            if g.len()<h.len() {return [Poly::zero(),g.clone()].to_vec();}
            let coeff=h.coeffs.last().unwrap().inverse();
            for i in (0..new_len).rev() {
                q[i]=coeff.clone()*rem[i+h.len()-1].clone();
                for j in 0..h.len() {
                    rem[i+j]=rem[i+j].clone()-q[i].clone()*h.coeffs[j].clone();
                }
                }  
    [Poly::new_from_coeffs(&q),Poly::new_from_coeffs(&rem)].to_vec()
    }
}