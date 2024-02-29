use core::fmt;
use std::{cmp::{max, min}, fmt::Display, ops::{Add, Div, Mul, Neg, Rem, Sub}};
use num_traits::{Zero, One};

use crate::field::Field;
#[derive(Clone, PartialEq, Eq, Default)]
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

#[derive(Clone, PartialEq, Eq, Default)]
pub struct MultivariatePoly<F:Field>(Vec<(F,Monomial)>);
impl <F:Field>MultivariatePoly<F> {
    pub fn new(terms:Vec<(F,Monomial)>)->Self{
        let mut new_terms:Vec<(F,Monomial)>= terms.into_iter().skip_while(|x| x.0.is_zero()).collect();
    MultivariatePoly(new_terms)


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