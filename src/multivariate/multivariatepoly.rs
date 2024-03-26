//! This library contains utilities for Multivariate polynomial.
//! Good references for algorithms below is 
//! [https://scholar.google.it/scholar_url?url=https://www.mdpi.com/2227-7390/7/5/441/pdf&hl=it&sa=X&ei=jRjnZeHFOZWty9YP9Me4iAs&scisig=AFWwaea_Q77frjP2J8Auw8F8Tfl-&oi=scholarr]
//! [David A. Cox John Little Donal O'Shea Ideals,Varieties, and Algorithms]

use core::fmt;
use std::{collections::BinaryHeap, fmt::Display, ops::{Add, Div, Mul, Neg, Rem, Sub}};
use crate::field::Field;
use super::terms::Terms;

#[derive(Clone, PartialEq,Eq, Default,Debug)]
pub struct MultivariatePoly<F:Field>{
    terms:Vec<Terms<F>>
}
impl <F:Field> Ord for MultivariatePoly<F> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.terms.cmp(&rhs.terms)
    }    
}
impl <F:Field>PartialOrd for MultivariatePoly<F> {
    fn partial_cmp(&self,rhs:&Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
impl <F:Field>MultivariatePoly<F> {
    pub fn new(terms:Vec<Terms<F>>)->Self{
        if terms.len()==1 {return MultivariatePoly{terms};}
        let mut new_terms:Vec<Terms<F>>= terms.iter().map(|x|Terms::new(x.coeff.clone(),x.multi_index.clone())).collect();
        new_terms= new_terms.into_iter().skip_while(|x| x.is_zero()).collect();
        new_terms.sort_by(|a,b|b.multi_index.cmp(&a.multi_index));
        let mut copy=new_terms.clone();
        let mut terms:Vec<Terms<F>>=Vec::new();
        for item in new_terms {
            let a =copy.iter().fold(Terms::zero(&item), |acc, x| {
                if x.multi_index== item.multi_index { &acc+x } else { acc}
        });
        copy.retain(|x| x.multi_index!=item.multi_index);
        terms.push(a);

    }
    terms.retain(|x|!x.coeff.is_zero());
    MultivariatePoly{terms}
    }
    pub fn is_empty(&self)->bool{
        self.terms.is_empty()
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
        self.is_empty()||*self==self.zero()||self.leading_term().is_zero()
    }
    pub fn div_rem (&self, rhs:&mut Self) -> (MultivariatePoly<F>,MultivariatePoly<F> ){

        if rhs.is_zero(){panic!("Cannot divide by zero poly!")}
        let mut q=MultivariatePoly::zero(&self);
        let mut r = MultivariatePoly::zero(&self);
        let mut lt=(&(self-&(&mut q*rhs))-&r).leading_term();
        while !lt.is_zero(){
            if lt.is_divisible_by(&mut rhs.leading_term()){
                let t= &mut lt/ &mut rhs.leading_term();
                q=&q+&MultivariatePoly::new(vec![t]);
            }
            else{
               r=r+MultivariatePoly::new(vec![lt.clone()]);
            }
            let x =&(self-&(&mut q*rhs))-&r;
            if x.is_zero(){break;}
            lt =x.leading_term();
        }
        (q,r)
    }
/// # Example
/// This algorithm express a multivariate polynomial f in the form
/// f = q_1*f_1 + ...+ q_s*f_s + r, where the quotients q_1,..,q_s  
/// and remainder r lie in k[x1, . . . , x n]. For details see 
/// [David A. Cox John Little Donal O'Shea Ideals,Varieties, and Algorithms,chapter 2]
/// 
/// ```
/// 
/// #[macro_use] extern crate algebra;
/// use num_bigint::BigInt;
/// use std::ops::Neg;
/// use algebra::intmod::PrimeField;
/// use algebra::multivariate::multivariatepoly::MultivariatePoly;
/// use algebra::multivariate::terms::Terms;
/// use algebra::multivariate::multiindex::MultiIndex;
/// 
/// let z13=PrimeField(BigInt::from(13));
/// let index0=MultiIndex::new(&vec![2,1]);
/// let index1=MultiIndex::new(&vec![1,2]);
/// let index2=MultiIndex::new(&vec![0,2]);
/// let index3=MultiIndex::new(&vec![0]);
/// let index4=MultiIndex::new(&vec![1,1]);
/// let term0=Terms::new(z13.one(), index0);//x_0^2*x_1
/// let term1=Terms::new(z13.one(), index1);//x_0*x_1^2
/// let term2=Terms::new(z13.one(), index2);//x_1^2
/// let term3=Terms::new(z13.one().neg(), index3);//-1
/// let term4=Terms::new(z13.one(), index4); //x_0x_1
/// let mut m1= MultivariatePoly::new(vec![term0.clone(),term1.clone(),term2.clone()]);//x_0^2*x_1+x_0*x_1^2+x_1^2
/// let m2= MultivariatePoly::new(vec![term4.clone(),term3.clone()]);//x_0*x_1-1
/// let m3= MultivariatePoly::new(vec![term2.clone(),term3.clone()]);//x_1^2-1
/// let multi_div= m1.multi_division_reminder(vec![m2,m3]);
/// for item in multi_div.0 {
/// println!("quotient is {}",item);
/// }
/// println!("reminder is {}",multi_div.1);
/// 
/// 
    pub fn multi_division_reminder(&mut self, mut divisors:Vec<Self>)->(Vec<Self>,Self){
        let mut quotients:Vec<MultivariatePoly<F>> = vec![MultivariatePoly::zero(&self);divisors.len()];
        let mut r = MultivariatePoly::zero(&self);
        let mut p=self.clone();
        while !p.is_zero() {
            let mut i=0usize;
            let mut division_occured = false;
            while i<divisors.len() && division_occured==false {
                    if p.leading_term().is_divisible_by(&mut divisors[i].leading_term()) {
                    let mut leading_terms_div = MultivariatePoly::new(vec![&mut (p.leading_term())/&mut (divisors[i].leading_term())]);
                    quotients[i]=&quotients[i]+&leading_terms_div;
                    p=&p-&(&mut leading_terms_div*&mut divisors[i]);
                    division_occured=true;

                } else {
                    i+=1;
                }
            }
                if !division_occured{
                    let lt = MultivariatePoly::new(vec![p.leading_term()]);
                    r=&r+&lt;
                    p=&p-&lt;
                }
        }

    (quotients,r)
    }
    
}
impl <F:Field>Display for MultivariatePoly<F>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str:String;
        let mut s:Vec<String>=Vec::new();
        if self.terms.is_empty(){return write!(f,"{}","Null");}
        for i in 0..self.terms.len() {
            if self.terms[i].is_zero(){continue;}
                str=format!("{}",self.terms[i]);
                s.push(str);
            }
            
        write!(f, "{}", s.join("+"))
    }
}
impl <F:Field>Add for MultivariatePoly<F>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
            &self+&rhs
    }
}
impl<'a, 'b, F:Field> Add<&'b mut MultivariatePoly<F>> for &'b mut MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_zero() {return rhs.clone()}
        if rhs.is_zero() {return self.clone()}
        let mut i= 0usize;
        let mut j= 0usize;
        let mut k= 0usize;
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
            k=k+1;
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
            k=k+1;
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
impl <F:Field> Mul <MultivariatePoly<F>> for MultivariatePoly<F>{
    type Output = Self;
    fn mul(self, rhs: MultivariatePoly<F>) -> Self::Output {
        &self*&rhs
    }
}

impl <'a,'b, F:Field> Mul <&'b MultivariatePoly<F>> for &'b  MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn mul(self, rhs:Self) -> Self::Output {
        let mut x = self.clone();
        let mut y= rhs.clone();
        &mut x*&mut y
        }
}
impl <'a,'b, F:Field> Mul<&'b mut MultivariatePoly<F>> for &'b mut MultivariatePoly<F> {
    type Output = MultivariatePoly<F>;
    fn mul(self, rhs:Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
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
            out = &out+&c[i];
        }
        out
        }
    
}

impl <'a,F:Field> Neg for &'a MultivariatePoly<F> {
    type Output = MultivariatePoly<F>;
    fn neg(self) -> Self::Output {
        let terms=self.terms.iter().map(|x| Terms::new(x.coeff.clone().neg(),x.multi_index.clone())).collect();
        MultivariatePoly::new(terms)
    }
    
}
impl <F:Field> Sub for MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        self+rhs.neg()
    }
}
impl <'a,'b,F:Field> Sub<&'b MultivariatePoly<F>> for &'b MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        self+&rhs.neg()
    }
}
impl <'a,'b,F:Field> Sub<&'b mut MultivariatePoly<F>> for &'b mut MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        self+&mut rhs.neg()
    }
}
impl <'a,'b, F:Field> Div<&'b mut MultivariatePoly<F>> for &'b MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn div(self, mut rhs: &'b mut MultivariatePoly<F>) -> Self::Output {
        self.div_rem(&mut rhs).0
    }
}
impl <'a,'b, F:Field> Div<&'b mut MultivariatePoly<F>> for &'b mut MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn div(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).0
    }
}
impl <'a,'b, F:Field> Rem<&'b mut MultivariatePoly<F>> for &'b MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn rem(self, mut rhs: &'b mut MultivariatePoly<F>) -> Self::Output {
        self.div_rem(&mut rhs).1
    }
}