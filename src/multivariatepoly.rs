use core::fmt;
use std::{cmp::{max, min}, fmt::Display, ops::{Add, Div, Mul, Neg, Rem, Sub}, process::Output};
use num_traits::{Zero, One};

use crate::{field::Field, multivariatepoly};
#[derive(Clone, PartialEq, Eq,PartialOrd,Ord,Default,Debug)]
pub struct Monomial(Vec<usize>);
impl Monomial{
    pub fn new_from_multi_index(multi_index:Vec<usize>)->Self {
        let mut new_multi_index:Vec<usize>= multi_index.into_iter().rev().skip_while(|&x| x == 0).collect();
    new_multi_index.reverse();
        Monomial(new_multi_index)
    }
    
    pub fn weight(&self)->usize{
        self.0.iter().sum()
     }
    pub fn multi_index(&self)->(Vec<usize>){
        self.0.clone()
    }
    pub fn print_monomial(&self)->String{
        let print:String=String::new();
        let mut s:Vec<String>=Vec::new();
        let mut str:String;
        for i in 0..self.0.len(){
            if self.0[i] ==0 {continue;}
            if self.0[i]==1{
                str=format!("x_{}",i);

            }
            else {
                str =(format!("x_{}^{}",i,self.0[i]));
            }
            if i!=self.0.len()-1{
                str = format!("{}*",str);
            }
        
            s.push(str);
        }
        
       s.concat()
    }
    
}
impl Display for Monomial{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{}", Monomial::print_monomial(self))
    }
}

#[derive(Clone, PartialEq, Eq, Default,Debug)]
pub struct MultivariatePoly<F:Field>(Vec<(F,Monomial)>);
impl <F:Field>MultivariatePoly<F> {
    pub fn new(terms:Vec<(F,Monomial)>)->Self{
        let mut new_terms:Vec<(F,Monomial)>= terms.into_iter().skip_while(|x| x.0.is_zero()).collect();
        new_terms.sort_by(|a,b|b.1.cmp(&a.1));
        MultivariatePoly(new_terms)
    }
    pub fn leading_coeff(&self)->F {
        self.0[0].0.clone()
        
    }
    pub fn leading_term(&self)->Self {
        Self::new(vec![self.0[0].clone()])
    }
    pub fn non_zero_elements(&self)->usize{
        self.0.len()
    }
    pub fn vector_terms(&self)->Vec<(F,Monomial)>{
        self.0.clone()
    }
    
}
impl <F:Field>Display for MultivariatePoly<F>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str=String::new();
        let mut s:Vec<String>=Vec::new();
        for i in 0..self.0.len() {
            str=format!("{}*{}",self.0[i].0,self.0[i].1);
            if i!=self.0.len()-1 {
                str=format!("{}+",str);
            }
            s.push(str);
        }
        write!(f, "{}", s.concat())
    }
}
impl <F:Field>Add for MultivariatePoly<F>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut i:usize = 0;
        let mut j:usize = 0;
        let mut k:usize = 0;
        let mut sum_terms:Vec<(F,Monomial)>=Vec::new();
        println!("non zero of p:{}",self.non_zero_elements());
        while i<self.non_zero_elements() && j<rhs.non_zero_elements() {
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
        while i<self.non_zero_elements(){
            sum_terms.push(self.vector_terms()[i].clone());
            i+=1;k+=1;
        }
        while j<rhs.non_zero_elements(){
            sum_terms.push(rhs.vector_terms()[j].clone());
            j+=1;k+=1;
        }
        MultivariatePoly::new(sum_terms)
    }
}