use core::fmt;
use std::{collections::BinaryHeap, fmt::Display, ops::{Add, Div, Mul, Neg, Rem, Sub}};


use num_traits::{One, Zero};

use crate::field::Field;
#[derive(Clone, PartialEq, Eq,PartialOrd,Ord,Default,Debug)]
pub struct Monomial(Vec< usize>);
impl<'a, 'b> Mul<&'b mut Monomial> for &'a mut Monomial {
    type Output = Monomial;
    fn mul(self:&'a mut Monomial, rhs: &'b mut Monomial) -> Self::Output {
        while self.0.len()<rhs.0.len() {
            self.0.append(&mut vec![0usize]);
        }
        while self.0.len()>rhs.0.len() {
            rhs.0.append(&mut vec![0usize]);
        }
        let mut sum=Vec::new();
        for (a, b) in self.0.iter().zip(rhs.0.iter()) {
            sum.push(a + b);
        }
        Monomial::new_from_multi_index(sum)
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
    pub fn is_zero(&self)->bool{
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
            /* if i!=self.0.len()-1{
                str = format!("{}*",str);
            } */
        
            s.push(str);
        }
       s.join("*")
       /* s.concat() */
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
        let mut index: Vec<usize> = vec![];
        index = (0..self.number_of_terms()).into_iter().map(|_| 0usize).collect();
        //initialize heap(a,B1)
        let mut heap: BinaryHeap<MultivariatePoly<F>> = BinaryHeap::new();
        for i in 0..self.number_of_terms() {
            let coeff = self.0[i].0.clone()*rhs.0[0].0.clone();
            let monomial=& mut self.0[i].1*& mut rhs.0[0].1;
            heap.push(MultivariatePoly::new(vec![(coeff,monomial)]));
        }
        println!("heap initialization is {:?}",heap);

        while !heap.is_empty() { 
            println!("entered in the while, so heap is not empty. It is {:?}",heap);
            if gamma !=c[k].0[0].1 && !c[k].0[0].0.is_zero() { 
                c.push(self.zero());
                k+=1;
            }
            c[k]=c[k].clone()+heap.pop().unwrap();
            println!("c[{k}] is {:?}",c[k].0);
            println!("removed an element from the heap. Now heap is {:?}",heap);
            println!("s is {:?}",s);
            println!("index[s] is {:?}",index[s]);
            index[s]+=1;

            if index[s]<rhs.number_of_terms() {
                //insert A_s*B_fs into the heap
                println!("Entered in the if. s is {}, index[{s}] is {}",s, index[s]);
                println!("A_s is {:?}, B_fs is {:?}",self.0[s],rhs.0[index[s]]);
                let coeff = self.0[s].0.clone()*rhs.0[index[s]].0.clone();
                let monomial=&mut self.0[s].1*& mut rhs.0[index[s]].1;
                heap.push(MultivariatePoly::new(vec![(coeff,monomial)]));
                println!("heap.push terms A_s*B_fs. New heap is {:?}",heap);

            }
            let x= heap.peek();
            if x.is_none() {continue;} 
            println!("x is {:?}",x);

            let aux_heap =heap.clone().into_sorted_vec();
            
            s= aux_heap.binary_search(&x.unwrap()).unwrap();        
            println!("aux_heap is {:?}",aux_heap);
        
            println!("s is {}",s);
            gamma=x.unwrap().0[0].1.clone();
            println!("gamma is {}",gamma);

        }
        /* if c[k]==self.zero() {
            k=k-1;        }
        println!("k is {}",k); */
        println!("c is {:?}",c);
        let mut out : MultivariatePoly<F>=self.zero();
        for i in c {
            println!("c[i] {:?}",i);

            out = out+i.clone()
        }
        println!("out {:?}",out.0);
        out
        }
    
}