//! This library contains utilities for Multivariate polynomial.
//! Good references for algorithms below is 
//! [https://scholar.google.it/scholar_url?url=https://www.mdpi.com/2227-7390/7/5/441/pdf&hl=it&sa=X&ei=jRjnZeHFOZWty9YP9Me4iAs&scisig=AFWwaea_Q77frjP2J8Auw8F8Tfl-&oi=scholarr]
//!
use core::fmt;
use std::{collections::BinaryHeap, fmt::Display, iter::Sum, ops::{Add, DerefMut, Div, Mul, Neg, Rem, Sub}};
use num_traits::{One, Zero};
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
        let mut new_terms:Vec<Terms<F>>= terms.into_iter().skip_while(|x| x.is_zero()||x.multi_index.is_zero()).collect();
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
    pub fn is_constant_multipoly(&self)->bool{
        todo!()    
}
    
}
impl <F:Field>Display for MultivariatePoly<F>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str=String::new();
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
impl <'a,'b, F:Field> Div<&'b mut MultivariatePoly<F>> for &'b mut MultivariatePoly<F>{
    type Output = MultivariatePoly<F>;
    fn div(self, rhs: Self) -> Self::Output {

        if rhs.is_zero(){panic!("Cannot divide by zero poly!")}
        let mut q=MultivariatePoly::zero(&self);
        let mut r = MultivariatePoly::zero(&self);
        let mut lt=(&(self.clone()-&mut q*rhs)-&r).leading_term();
        while !lt.is_zero(){
            if lt.is_divisible_by(&mut rhs.leading_term()){
                let t= &mut lt/ &mut rhs.leading_term();
                q=&q+&MultivariatePoly::new(vec![t]);
            }
            else{
               r=r+MultivariatePoly::new(vec![lt.clone()]);
            }
            let x =(&(self.clone()-&mut q*rhs)-&r);
            if x.is_zero(){break;}
            lt =x.leading_term();
        }
        q
    }
}