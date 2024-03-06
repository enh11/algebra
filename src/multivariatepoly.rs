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

#[derive(Clone, PartialEq,Eq, PartialOrd, Ord, Default,Debug)]
pub struct MultivariatePoly<F:Field>{
    terms:Vec<Terms<F>>
}
impl <F:Field>MultivariatePoly<F> {
    pub fn new(terms:Vec<Terms<F>>)->Self{
        if terms.len()==1 {return MultivariatePoly{terms};}
        
        let mut new_terms:Vec<Terms<F>>= terms.into_iter().skip_while(|x| x.is_zero()||x.multi_index.is_zero()).collect();
        new_terms.sort_by(|a,b|b.multi_index.cmp(&a.multi_index));
        MultivariatePoly{terms:new_terms}
    }
    pub fn leading_coeff(&self)->F {
        self.terms[0].coeff.clone()
        
    }
    pub fn leading_term(&self)->Terms<F> {
        self.terms[0].clone()
    }
    pub fn number_of_terms(&self)->usize{
        self.terms.len()
    }
    pub fn zero(&self)->Self{
        MultivariatePoly::new(vec![Terms::zero(&self.leading_term())])
    }
    pub fn is_zero(&self)->bool{
        *self==self.zero()
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
        for i in 0..self.terms.len() {
                str=format!("{}",self.terms[i]);
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
        let mut sum_terms:Vec<Terms<F>>=Vec::new();
        while i<self.number_of_terms() && j<rhs.number_of_terms() {
            if self.terms[i].multi_index<rhs.terms[j].multi_index{
                sum_terms.push(rhs.terms[j].clone());
                j+=1;
            }
            else if self.terms[i].multi_index>rhs.terms[j].multi_index{
                sum_terms.push(self.terms[i].clone());
                i+=1;
            }
            else {
                let coeff=self.terms[i].coeff.clone()+rhs.terms[j].coeff.clone();
                sum_terms.push(Terms::new(coeff.clone(),self.terms[i].multi_index.clone()));
                i+=1;j+=1;
                if coeff.is_zero(){continue;}
            }
            k+=1;
        }
        while i<self.number_of_terms(){
            sum_terms.push(self.terms[i].clone());
            i+=1;k+=1;
        }
        while j<rhs.number_of_terms(){
            sum_terms.push(rhs.terms[j].clone());
            j+=1;k+=1;
        }
        MultivariatePoly::new(sum_terms)
    }
}
impl<'a, 'b, F:Field> Add<&'b MultivariatePoly<F>> for &'b  MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn add(self, rhs: &'b MultivariatePoly<F>) -> Self::Output {
        if self.is_zero() {return rhs.clone()}
        if rhs.is_zero() {return self.clone()}
        let mut i:usize = 0;
        let mut j:usize = 0;
        let mut k:usize = 0;
        let mut sum_terms:Vec<Terms<F>>=Vec::new();
        while i<self.number_of_terms() && j<rhs.number_of_terms() {
            if self.terms[i].multi_index<rhs.terms[j].multi_index{
                sum_terms.push(rhs.terms[j].clone());
                j+=1;
            }
            else if self.terms[i].multi_index>rhs.terms[j].multi_index{
                sum_terms.push(self.terms[i].clone());
                i+=1;
            }
            else {
                let coeff=self.terms[i].coeff.clone()+rhs.terms[j].coeff.clone();
                sum_terms.push(Terms::new(coeff.clone(),self.terms[i].multi_index.clone()));
                i+=1;j+=1;
                if coeff.is_zero(){continue;}
            }
            k+=1;
        }
        while i<self.number_of_terms(){
            sum_terms.push(self.terms[i].clone());
            i+=1;k+=1;
        }
        while j<rhs.number_of_terms(){
            sum_terms.push(rhs.terms[j].clone());
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
        let mut c: Vec<MultivariatePoly<F>>=vec![self.zero()];
        let mut k=0usize;
        let mut s = 0usize;
        let mut gamma=&mut self.terms[0].multi_index+&mut rhs.terms[0].multi_index;
        let mut index: Vec<usize>;
        index = (0..self.number_of_terms()).into_iter().map(|_| 0usize).collect();
        //initialize heap(a,B1)
        let mut heap: BinaryHeap<MultivariatePoly<F>> = BinaryHeap::new();
        for i in 0..self.number_of_terms() {
            let coeff = self.terms[i].coeff.clone()*rhs.terms[0].coeff.clone();
            let multi_index=& mut self.terms[i].multi_index+& mut rhs.terms[0].multi_index;
            heap.push(MultivariatePoly::new(vec![Terms::new(coeff,multi_index)]));
        }
        while !heap.is_empty() { 
            if gamma !=c[k].terms[0].multi_index && !c[k].terms[0].coeff.is_zero() { 
                c.push(self.zero());
                k+=1;
            }
            c[k]=c[k].clone()+heap.pop().unwrap();
            index[s]+=1;

            if index[s]<rhs.number_of_terms() {
                //insert A_s*B_fs into the heap
                let coeff = self.terms[s].coeff.clone()*rhs.terms[index[s]].coeff.clone();
                let term=&mut self.terms[s].multi_index+& mut rhs.terms[index[s]].multi_index;
                heap.push(MultivariatePoly::new(vec![Terms::new(coeff,term)]));

            }
            let x= heap.peek();
            if x.is_none() {continue;} 
            s=heap.clone().into_sorted_vec().binary_search(&x.unwrap()).unwrap();        
            gamma=x.unwrap().terms[0].multi_index.clone();
        }
        if c[k].is_zero(){k-=1}
        let mut out : MultivariatePoly<F>=self.zero();
        for i in 0..=k {
            out = &out+&c[i]
        }
        out
        }
    
}