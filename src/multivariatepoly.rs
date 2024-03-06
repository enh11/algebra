//! This library contains utilities for Multivariate polynomial.
//! Good references for algorithms below is 
//! [https://scholar.google.it/scholar_url?url=https://www.mdpi.com/2227-7390/7/5/441/pdf&hl=it&sa=X&ei=jRjnZeHFOZWty9YP9Me4iAs&scisig=AFWwaea_Q77frjP2J8Auw8F8Tfl-&oi=scholarr]
//!
use core::fmt;
use std::{collections::BinaryHeap, fmt::Display, ops::{Add, DerefMut, Div, Mul, Neg, Rem, Sub}};
use num_traits::{One, Zero};
use crate::field::Field;
#[derive(Clone,PartialEq, Eq,PartialOrd,Ord,Default,Debug)]
pub struct MultiIndex(Vec< usize>);
impl MultiIndex {
    pub fn new(multi_index:&Vec<usize>)->Self{
        MultiIndex(multi_index.to_vec())
    }
    pub fn resize<'a>(&'a mut self,rhs:&'a mut Self)->(&Self,&Self){
        while self.0.len()<rhs.0.len() {
            self.0.append(&mut vec![0usize]);

        }
        while self.0.len()>rhs.0.len() {
            rhs.0.append(&mut vec![0usize]);
        }
        (self,rhs)
    }
    pub fn len(&self)->usize {
        self.0.len()
        
    }
    pub fn weight(&self)->usize{
        self.0.iter().sum()
    }
    pub fn zero()->Self {
        MultiIndex::new(&vec![])
    }
    pub fn is_zero(&self)->bool{
        *self==Self::zero()||self.len()==self.0.iter().filter(|x| **x==0usize).count()
    }
    pub fn check_sub(&mut self,rhs:&mut Self)->bool{
        let (self_resize,rhs_resize)=self.resize(rhs);    
        let matching = self_resize.0.iter().zip(&rhs_resize.0).filter(|&(a, b)| a >= b).count();
        matching==self_resize.len()
    }
    
}
impl<'a, 'b> Add<&'b mut MultiIndex> for &'a mut MultiIndex {
    type Output = MultiIndex;
/// # Example
/// 
/// ```
/// use algebra::multivariatepoly::MultiIndex;
/// let mut multi_index_1 = MultiIndex::new(&vec![2,0]);
/// let mut multi_index_2=MultiIndex::new(&vec![1,0,1]);
/// let sum = &mut multi_index_1+&mut multi_index_2;
/// let expected_sum = MultiIndex::new(&vec![3,0,1]);
/// assert_eq!(sum,expected_sum);
///  
/// ```
    fn add(self:&'a mut MultiIndex, rhs: &'b mut MultiIndex) -> Self::Output {
        if self.is_zero() {return rhs.clone();}
        if rhs.is_zero() {return self.clone();}
        let (self_resize,rhs_resize)=self.resize(rhs);
        let mut sum=Vec::new();
        for (a, b) in self_resize.0.iter().zip(rhs_resize.0.iter()) {
            sum.push(a + b);
        }
        MultiIndex(sum)
        
    }
}
impl<'a, 'b> Sub<&'b mut MultiIndex> for &'a mut MultiIndex {
    type Output = MultiIndex;
/// # Example
/// 
/// ```
/// use algebra::multivariatepoly::MultiIndex;
/// let mut multi_index_1 = MultiIndex::new(&vec![2,2,3,0]);
/// let mut multi_index_2=MultiIndex::new(&vec![1,0,1]);
/// let sub = &mut multi_index_1-&mut multi_index_2;
/// let expected_sub = MultiIndex::new(&vec![1,2,2,0]);
/// assert_eq!(sub,expected_sub);
///  
/// ```
    fn sub(self:&'a mut MultiIndex, rhs: &'b mut MultiIndex) -> Self::Output {
        if !self.check_sub(rhs) {panic!("Cant subtract {:?} from {:?}",self,rhs)}
        let (self_resize,rhs_resize)=self.resize(rhs);
        let mut sub=Vec::new();
        for (a, b) in self_resize.0.iter().zip(rhs_resize.0.iter()) {
            sub.push(a - b);
        }
        MultiIndex(sub)
        
    }
}

#[derive(Clone, PartialEq, Eq,PartialOrd,Ord,Default,Debug)]
pub struct Terms<F:Field>{
    coeff:F,
    multi_index:MultiIndex
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

impl<'a, 'b,F:Field> Mul<&'b mut Terms<F>>for &'a mut Terms<F> {
    type Output = Terms<F>;
/// # Example
/// 
/// ```
/// 
/// use algebra::intmod::PrimeField;
/// use algebra::field::Field;
/// use num_bigint::BigInt;
/// use algebra::multivariatepoly::MultiIndex;
/// use algebra::multivariatepoly::Terms;
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
/// use algebra::multivariatepoly::MultiIndex;
/// use algebra::multivariatepoly::Terms;
/// let z13=PrimeField(BigInt::from(13));
///     let mut term1 = Terms::new(z13.new(BigInt::from(5)),MultiIndex::new(&vec![2,3,1]));
///     let mut term2 = Terms::new(z13.new(BigInt::from(4)),MultiIndex::new(&vec![2,1]));
///     let division= &mut term1 / &mut term2;
///     let expected_division=Terms::new(z13.new(BigInt::from(11)),MultiIndex::new(&vec![0,2,1]));
/// assert_eq!(division,expected_division);
/// 
/// ```
    fn div(self:&'a mut Terms<F>, rhs: &'b mut Terms<F>) -> Self::Output {    
        if !self.multi_index.check_sub(&mut rhs.multi_index) {panic!("Cannot divide!")}
        let coeff=self.coeff.clone()*rhs.coeff.inverse();
        let multi_index=&mut self.multi_index-&mut rhs.multi_index;
        
        Terms::new(coeff,multi_index)
        }
}




#[derive(Clone, PartialEq, Eq,PartialOrd,Ord,Default,Debug)]
pub struct Monomial(Vec< usize>);
impl<'a, 'b> Mul<&'b mut Monomial> for &'a mut Monomial {
    type Output = Monomial;
/// # Example
/// 
///  //https://scholar.google.it/scholar_url?url=https://www.mdpi.com/2227-7390/7/5/441/pdf&hl=it&sa=X&ei=jRjnZeHFOZWty9YP9Me4iAs&scisig=AFWwaea_Q77frjP2J8Auw8F8Tfl-&oi=scholarr
/// ```
/// 
/// use algebra::intmod::PrimeField;
/// use algebra::field::Field;
/// use num_bigint::BigInt;
/// use algebra::multivariatepoly::Monomial;
/// let z13=PrimeField(BigInt::from(13));
/// let mut mono1 = Monomial::new_from_multi_index(vec![2,0]);
/// let mut mono2=Monomial::new_from_multi_index(vec![1,0,1]);
/// let mul = &mut mono1*&mut mono2;
/// let expected_mul = Monomial::new_from_multi_index(vec![3,0,1]);
/// assert_eq!(mul,expected_mul);
///  
/// ```
    fn mul(self:&'a mut Monomial, rhs: &'b mut Monomial) -> Self::Output {
        let (self_resize,rhs_resize)=self.resize(rhs);

        let mut sum=Vec::new();
        for (a, b) in self_resize.iter().zip(rhs_resize.iter()) {
            sum.push(a + b);
        }
        Monomial::new_from_multi_index(sum)
    }
}

impl<'a, 'b> Div<&'b mut Monomial> for &'a mut Monomial {
    type Output = Monomial;
/// # Example
/// ```
/// use algebra::multivariatepoly::Monomial;
/// 
///     let mut mono1 = Monomial::new_from_multi_index(vec![2, 0,3,1]);
///     let mut mono2 = Monomial::new_from_multi_index(vec![1, 0, 1]);
///     let division= &mut mono1 / &mut mono2;
///     let expected_division=Monomial::new_from_multi_index(vec![1,0,2,1]);
/// assert_eq!(division,expected_division);
/// 
/// ```
    fn div(self:&'a mut Monomial, rhs: &'b mut Monomial) -> Self::Output {    
        if !self.is_divisible(rhs) {panic!("Cannot divide!")}
        let (self_resize,rhs_resize)=self.resize(rhs);
        let diff: Vec<usize>=self_resize.iter().zip(rhs_resize.iter()).map(|x|x.0-x.1).collect();
        Monomial::new_from_multi_index(diff)
        }
}

impl Monomial{
    pub fn new_from_multi_index(multi_index:Vec<usize>)->Self {
        if multi_index.len()==1 {Monomial(multi_index)}
        else{
        let mut new_multi_index:Vec<usize>= multi_index.into_iter().rev().skip_while(|&x| x == 0).collect();
        new_multi_index.reverse();
        Monomial(new_multi_index)}
        
    }
    pub fn is_divisible(&mut self,rhs:&mut Self)->bool{
        let (self_resize,rhs_resize)=self.resize(rhs);    
        let matching = self_resize.iter().zip(&rhs_resize).filter(|&(a, b)| a >= b).count();
        matching==self_resize.len()
    }
    pub fn resize(&mut self,rhs: &mut Self)->(Vec<usize>,Vec<usize>){
        while self.0.len()<rhs.0.len() {
            self.0.append(&mut vec![0usize]);

        }
        while self.0.len()>rhs.0.len() {
            rhs.0.append(&mut vec![0usize]);
        }
        (self.0.clone(),rhs.0.clone())
        
    }
    pub fn is_one(&self)->bool{
        self.0.is_empty()
    }
    pub fn is_constant(&self)->bool{
        self.0.is_empty()||(self.0.len()==1&&self.0[0].is_zero())
    }
    pub fn weight(&self)->usize{
        self.0.iter().sum()
     }
    pub fn multi_index(&self)->Vec<usize>{
        self.0.clone()
    }
    pub fn print_monomial(&self)->String{
        let _print:String=String::new();
        let mut s:Vec<String>=Vec::new();
        let mut str:String;
        for i in 0..self.0.len(){
            if self.0[i] ==0 {continue;}
            if self.0[i]==1{
                str=format!("x_{}",i);

            }
            else {
                str =format!("x_{}^{}",i,self.0[i]);
            }
        s.push(str);
        }
       s.join("*")
    }
    
}
impl Display for Monomial{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{}", Monomial::print_monomial(self))
    }
}

#[derive(Clone, PartialEq,Eq, PartialOrd, Ord, Default,Debug)]
pub struct MultivariatePoly<F:Field>(Vec<(F,Monomial)>);
impl <F:Field>MultivariatePoly<F> {
    pub fn new(terms:Vec<(F,Monomial)>)->Self{
        if terms.len()==1 {MultivariatePoly(terms)}
        else{
        let mut new_terms:Vec<(F,Monomial)>= terms.into_iter().skip_while(|x| x.0.is_zero()).collect();
        new_terms.sort_by(|a,b|b.1.cmp(&a.1));
        MultivariatePoly(new_terms)}
    }
    pub fn leading_coeff(&self)->F {
        self.0[0].0.clone()
        
    }
    pub fn leading_term(&self)->Self {
        Self::new(vec![self.0[0].clone()])
    }
    pub fn number_of_terms(&self)->usize{
        self.0.len()
    }
    pub fn vector_terms(&self)->Vec<(F,Monomial)>{
        self.0.clone()
    }
    pub fn zero(&self)->Self{
        let zero = F::zero(&self.0[0].0.zero());
        let zero_monomial=Monomial::new_from_multi_index(vec![0usize]);
        MultivariatePoly::new(vec![(zero,zero_monomial)])
    }
    pub fn is_zero(&self)->bool{
        self.0.is_empty()||self.leading_term().leading_coeff().is_zero()||self==&self.zero()
    }
    pub fn is_constant_multipoly(&self)->bool{
        todo!()    
}
    
}
impl <F:Field>Display for MultivariatePoly<F>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str=String::new();
        let mut s:Vec<String>=Vec::new();
        for i in 0..self.0.len() {
            if self.0[i].1.is_constant(){
                str=format!("{}",self.0[i].0)
            } else{
            str=format!("{}*{}",self.0[i].0,self.0[i].1);
            }
            s.push(str);
        }
        write!(f, "{}", s.join("+"))
    }
}
impl <F:Field>Add for MultivariatePoly<F>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_zero() {return rhs}
        if rhs.is_zero() {return self}
        let mut i:usize = 0;
        let mut j:usize = 0;
        let mut k:usize = 0;
        let mut sum_terms:Vec<(F,Monomial)>=Vec::new();
        while i<self.number_of_terms() && j<rhs.number_of_terms() {
            if self.vector_terms()[i].1.multi_index()<rhs.vector_terms()[j].1.multi_index(){
                sum_terms.push(rhs.vector_terms()[j].clone());
                j+=1;
            }
            else if self.vector_terms()[i].1.multi_index()>rhs.vector_terms()[j].1.multi_index(){
                sum_terms.push(self.vector_terms()[i].clone());
                i+=1;
            }
            else {
                let coef=self.0[i].0.clone()+rhs.0[j].0.clone();
                sum_terms.push((coef.clone(),self.vector_terms()[i].1.clone()));
                i+=1;j+=1;
                if coef.is_zero(){continue;}
            }
            k+=1;
        }
        while i<self.number_of_terms(){
            sum_terms.push(self.vector_terms()[i].clone());
            i+=1;k+=1;
        }
        while j<rhs.number_of_terms(){
            sum_terms.push(rhs.vector_terms()[j].clone());
            j+=1;k+=1;
        }
        MultivariatePoly::new(sum_terms)
    }
}
impl<'a, 'b, F:Field> Add<&'b MultivariatePoly<F>> for &'b  MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn add(self, rhs: Self) -> Self::Output {
    
        let mut i:usize = 0;
        let mut j:usize = 0;
        let mut k:usize = 0;
        let mut sum_terms:Vec<(F,Monomial)>=Vec::new();
        if self.is_zero(){return rhs.clone();}
        if rhs.is_zero(){return self.clone();}
        while i<self.number_of_terms() && j<rhs.number_of_terms() {
            if self.vector_terms()[i].1.multi_index()<rhs.vector_terms()[j].1.multi_index(){
                sum_terms.push(rhs.vector_terms()[j].clone());
                j+=1;
            }
            else if self.vector_terms()[i].1.multi_index()>rhs.vector_terms()[j].1.multi_index(){
                sum_terms.push(self.vector_terms()[i].clone());
                i+=1;
            }
            else {
                let coef=self.0[i].0.clone()+rhs.0[j].0.clone();
                sum_terms.push((coef.clone(),self.vector_terms()[i].1.clone()));
                i+=1;j+=1;
                if coef.is_zero(){continue;}
            }
            k+=1;
        }
        while i<self.number_of_terms(){
            sum_terms.push(self.vector_terms()[i].clone());
            i+=1;k+=1;
        }
        while j<rhs.number_of_terms(){
            sum_terms.push(rhs.vector_terms()[j].clone());
            j+=1;k+=1;
        }
        MultivariatePoly::new(sum_terms)
    }
}
impl <'a,'b, F:Field> Mul for MultivariatePoly<F> {
    type Output = Self;
    fn mul(mut self, mut rhs:Self) -> Self::Output {
        if self.number_of_terms().is_zero() || rhs.number_of_terms().is_zero() {
            return self.zero();}
        let mut c=vec![self.zero()];
        let mut k=0usize;
        let mut s = 0usize;
        let mut gamma=&mut self.0[0].1*&mut rhs.0[0].1;
        let mut index: Vec<usize>;
        index = (0..self.number_of_terms()).into_iter().map(|_| 0usize).collect();
        //initialize heap(a,B1)
        let mut heap: BinaryHeap<MultivariatePoly<F>> = BinaryHeap::new();
        for i in 0..self.number_of_terms() {
            let coeff = self.0[i].0.clone()*rhs.0[0].0.clone();
            let monomial=& mut self.0[i].1*& mut rhs.0[0].1;
            heap.push(MultivariatePoly::new(vec![(coeff,monomial)]));
        }
        while !heap.is_empty() { 
            if gamma !=c[k].0[0].1 && !c[k].0[0].0.is_zero() { 
                c.push(self.zero());
                k+=1;
            }
            c[k]=c[k].clone()+heap.pop().unwrap();
            index[s]+=1;

            if index[s]<rhs.number_of_terms() {
                //insert A_s*B_fs into the heap
                let coeff = self.0[s].0.clone()*rhs.0[index[s]].0.clone();
                let monomial=&mut self.0[s].1*& mut rhs.0[index[s]].1;
                heap.push(MultivariatePoly::new(vec![(coeff,monomial)]));

            }
            let x= heap.peek();
            if x.is_none() {continue;} 
            s=heap.clone().into_sorted_vec().binary_search(&x.unwrap()).unwrap();        
            gamma=x.unwrap().0[0].1.clone();
        }
        let mut out : MultivariatePoly<F>=self.zero();
        for i in c {
            out = out+i
        }
        out
        }
    
}