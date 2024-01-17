use core::fmt;
use std::{ops::{Add, Sub, Mul, Div, Rem, Neg}, cmp::max, slice::Windows};
use num_bigint::BigInt;
use num_traits::{Zero, One};

use crate::field::Field;
#[derive(Debug, PartialEq, Clone, Eq,PartialOrd,Ord)]
pub struct Poly<T> {
    pub coeffs: Vec<T>,
}
impl <F:Field>fmt::Display for Poly<F> {
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
    pub fn remove_last_zeros(vec:&mut Vec<F>)->Vec<F>{
        while vec.last().is_some() && vec.last().unwrap().is_zero() {
            vec.remove(vec.len()-1);}
            vec.to_vec()
        
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
    pub fn one(&self)->Poly<F>{
        let coeff=[self.coeffs[0].one()];
        Poly::new_from_coeffs(&coeff)
    }
    pub fn zero(&self)->Poly<F> {
        let coeff=[self.coeffs[0].zero()];
        Poly::new_from_coeffs(&coeff)
    } 
    pub fn is_zero(&self)->bool{
        self.coeffs.is_empty()
    }
    pub fn is_one(&self)->bool{
        self==&self.one()
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
        if rhs.is_zero(){return self.clone();}
        if self.is_zero(){return rhs.clone();}
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
        sum=Poly::remove_last_zeros(&mut sum);
Poly::new_from_coeffs(&sum)
    }
}
impl <F:Field> Neg for Poly<F> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.is_zero(){return self.clone();}
        for i in 0..self.len(){
            self.coeffs[i]=self.coeffs[i].clone().neg()
        }
    
    Poly::new_from_coeffs(&self.coeffs)
    }
    
}
impl <'a,'b,F:Field>Sub<&'b Poly<F>>for  &'b Poly<F> {
    type Output=Poly<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.is_zero(){return self.clone();}
        if self.is_zero(){return rhs.clone().neg();}
        let max_len=max(self.coeffs.len(), rhs.coeffs.len());
        let mut v1=self.coeffs.clone();
        let mut v2=rhs.coeffs.clone();
        if self.len()<rhs.len(){
            v1=self.extend(max_len);
        } else if self.len()>rhs.len(){
            v2=rhs.extend(max_len);
        }
        let mut sub=Vec::with_capacity(v1.len());
        for i in 0..v1.len(){
            sub.push(v1[i].clone()-v2[i].clone());
        }
    sub=Poly::remove_last_zeros(&mut sub);
Poly::new_from_coeffs(&sub)
    }
}
impl <'a,'b,F:Field>Mul<&'b Poly<F>>for  &'b Poly<F>{
    type Output=Poly<F>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero(){return rhs.clone()}
        if rhs.is_zero(){return self.clone();}
        let zero=self.coeffs[0].clone().zero();
        let new_len=self.coeffs.len()+rhs.coeffs.len()-1;
        let mut prod=vec![zero;new_len];
        for i in 0..self.len(){
            for j in 0..rhs.len(){
                prod[i+j]=prod[i+j].clone()+self.coeffs[i].clone()*rhs.coeffs[j].clone();
            }
        }
        prod=Poly::remove_last_zeros(&mut prod);
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
impl <F:Field>Poly<F> {
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
impl <F:Field> Poly<F>{
    pub fn div_rem(g:&Poly<F>,h:&Poly<F>)->Vec<Poly<F>>{
        let zero=g.coeffs[0].clone().zero();
        let mut rem=g.coeffs.clone();
        if g.len()<h.len() {return [g.zero(),g.clone()].to_vec();}
        let new_len=g.len()-h.len()+1;
        let mut q: Vec<F>=vec![zero;new_len];
        let coeff=h.coeffs.last().unwrap().inverse();
        for i in (0..new_len).rev() {
            q[i]=coeff.clone()*rem[i+h.len()-1].clone();
            for j in 0..h.len() {
                rem[i+j]=rem[i+j].clone()-q[i].clone()*h.coeffs[j].clone();
            }
            }
            rem=Poly::remove_last_zeros(&mut rem);
            q=Poly::remove_last_zeros(&mut q);
[Poly::new_from_coeffs(&q),Poly::new_from_coeffs(&rem)].to_vec()
}
}
impl <F:Field> Poly<F>{
pub fn evaluation(&self,alpha:&F)->F {
    let mut value = self.coeffs[0].zero();
    for i in (0..self.len()).rev(){
        value=value*alpha.clone()+self.coeffs[i].clone();
    }
    value
}
pub fn normal_poly(&mut self)->Poly<F> {
    let c=self.coeffs.last().unwrap().inverse();
    for i in 0..self.len(){self.coeffs[i]=self.coeffs[i].clone()*&c}
    Poly::new_from_coeffs(&self.coeffs)
    
}
pub fn multiple(&mut self, alpha:&F)->Poly<F>{
    for i in 0..self.len(){
    self.coeffs[i]=self.coeffs[i].clone()*alpha;}
    Poly::new_from_coeffs(&self.coeffs)
}
pub fn gcdext(g:&Poly<F>,h:&Poly<F>)->Vec<Poly<F>>{
    let mut r0=g.clone(); let mut r1=h.clone();
    let mut s0=g.one(); let mut s1: Poly<F>=h.zero();
    let mut t0: Poly<F>=g.zero(); let mut t1=g.one();
    let mut _div_rem:Vec<Poly<F>>;
    while !r1.is_zero(){
        let div_rem=Poly::div_rem(&r0,&r1);
        r0=r1;
        r1=div_rem[1].clone();
        let aux_s0=s0.clone();
        s0=s1.clone();
        let aux_t0=t0.clone();
        t0=t1.clone(); 
        s1=&aux_s0-&(&s1*&div_rem[0].clone());
        t1=&aux_t0-&(&t1*&div_rem[0].clone());
    }
    
    let d=Poly::new_from_coeffs(&r0.coeffs);
    let u=Poly::new_from_coeffs(&s0.coeffs);
    let v=Poly::new_from_coeffs(&t0.coeffs);
    [u,v,d].to_vec()
}
pub fn is_coprime(g:&Poly<F>,h:&Poly<F>)->bool{
    Poly::gcdext(g, h)[2].is_one()
}
}