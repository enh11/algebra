use core::fmt;
use std::{ops::{Add, Sub, Mul, Div, Rem, Neg}, cmp::{max, min}};
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
    pub fn new_from_coeffs(coeffs:&[F])->Self {
        let new_coeffs=Poly::remove_zeros(&mut coeffs.to_vec());
        Poly { coeffs: new_coeffs } 
    }
    pub fn len(&self)->usize{
        self.coeffs.len()
    }
    pub fn deg(&self) -> usize {
        if self.len().is_zero(){println!("found vector of len zero{:?}",self.coeffs);}
        self.coeffs.len()-1
    }
    pub fn is_constant(&self)->bool {
        self.len().is_one()
    }
    pub fn remove_zeros( vec:&mut Vec<F>)->Vec<F>{
        loop {
            if vec.len().is_one(){break vec.to_vec();}
            if vec.first().is_some() && vec.first().unwrap().is_zero() {
            vec.remove(0);} else {break vec.to_vec()}}
        /* while vec.last().is_some() && vec.last().unwrap().is_zero() {
            vec.remove(vec.len()-1);}*/        
    }
    pub fn extend(&self,zero_to_add:usize)->Vec<F> {
        let zero=self.coeffs[0].zero();
        let mut new_coeffs=vec![zero;self.len()+zero_to_add];
            for i in 0..self.len() {
                new_coeffs[i+zero_to_add]=self.coeffs[i].clone()
            }
        new_coeffs
        
    } 
    pub fn one(&self)->Poly<F>{
        let mut coeff=[self.coeffs[0].one()];
        Poly::new_from_coeffs(&mut coeff)
    }
    pub fn zero(&self)->Poly<F> {
        let mut coeff=[self.coeffs[0].zero()];
        Poly::new_from_coeffs(&mut coeff)
    } 
    pub fn is_zero(&self)->bool{
        self==&self.zero()
    }
    pub fn is_one(&self)->bool{
        self==&self.one()
    }
}

impl <'a,'b,F:Field>Add<&'b Poly<F>>for  &'b Poly<F> {
    type Output=Poly<F>;
/// # Example
/// ```
/// #[macro_use] extern crate algebra;
/// use num_bigint::BigInt;
/// use algebra::poly::Poly;
/// use algebra::intmod::{Mod,PrimeField};
/// use algebra::field::Field;
/// let prime_base=BigInt::from(13);
/// let z13=PrimeField(Some(BigInt::from(13)));
/// let p1 = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3)));
/// let p2 = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(2)));
/// println!("poly p1 is {}",p1);
/// println!("poly p2 is {}",p2);
/// println!("poly sum is {}",&p1+&p2);
/// println!("poly sub is {}",&p1-&p2);
/// println!("poly mul is {}",&p1*&p2);
/// let add = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(7)),z13.new(BigInt::from(5)));
/// let sub = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(10)),z13.new(BigInt::from(1)));
/// let prod = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(12)),z13.new(BigInt::from(6)),z13.new(BigInt::from(6)));
/// assert_eq!(add,&p1+&p2);
/// assert_eq!(sub,&p1-&p2);
/// assert_eq!(prod,&p1*&p2);
/// ```
/// 
    fn add(self, rhs: Self) -> Self::Output {
        let zero=self.coeffs[0].clone().zero();
        if rhs.is_zero(){return self.clone();}
        if self.is_zero(){return rhs.clone();}
        let zero_to_add=max(self.coeffs.len(), rhs.coeffs.len())-min(self.coeffs.len(),rhs.coeffs.len());
        let mut v1=self.coeffs.clone();
        let mut v2=rhs.coeffs.clone();
        if self.len()<rhs.len(){
            
            v1=self.extend(zero_to_add);

        } else if self.len()>rhs.len(){
            v2=rhs.extend(zero_to_add);
        }
        let mut sum=vec!(zero;v1.len());
        for i in 0..v1.len(){
            sum.push(v1[i].clone()+v2[i].clone());
        }
Poly::new_from_coeffs(&mut sum)
    }
}
impl <F:Field> Neg for Poly<F> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.is_zero(){return self.clone();}
        for i in 0..self.len(){
            self.coeffs[i]=self.coeffs[i].clone().neg()
        }
    
    Poly::new_from_coeffs(&mut self.coeffs)
    }
    
}
impl <'a,'b,F:Field>Sub<&'b Poly<F>>for  &'b Poly<F> {
    type Output=Poly<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        let zero=self.coeffs[0].clone().zero();
        if rhs.is_zero(){return self.clone();}
        if self.is_zero(){return rhs.clone().neg();}
        let zero_to_add=max(self.len(), rhs.len())-min(self.len(),rhs.len());
        let mut v1=self.coeffs.clone();
        let mut v2=rhs.coeffs.clone();
        if self.len()<rhs.len(){
            v1=self.extend(zero_to_add);
        } else if self.len()>rhs.len(){
            v2=rhs.extend(zero_to_add);
        }
        let mut sub=vec![zero;v1.len()];
        for i in 0..v1.len(){
            sub[i]=v1[i].clone()-v2[i].clone();
        }
Poly::new_from_coeffs(&mut sub)
    }
}
impl <'a,'b,F:Field>Mul<&'b Poly<F>>for  &'b Poly<F>{
    type Output=Poly<F>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero()||rhs.is_zero(){return rhs.zero()}
        if self.is_one(){return rhs.clone();}
        if rhs.is_one(){return self.clone();}
        let zero=self.coeffs[0].clone().zero();
        let new_len=self.coeffs.len()+rhs.coeffs.len()-1;
        let mut prod=vec![zero;new_len];
        for i in 0..self.len(){
            for j in 0..rhs.len(){
                prod[i+j]=prod[i+j].clone()+self.coeffs[i].clone()*rhs.coeffs[j].clone();
            }
        }
Poly::new_from_coeffs(&mut prod.to_vec())
    }
}
impl <'a,'b,F:Field + std::fmt::Debug>Div<&'b Poly<F>>for  &'b Poly<F>{
    type Output=Poly<F>;
    fn div(self, rhs: &'b Poly<F>) -> Self::Output{
        Poly::div_rem(self, rhs)[0].clone()
}
}
impl <'a,'b,F:Field + std::fmt::Debug>Rem<&'b Poly<F>>for  &'b Poly<F>{
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
            let term = if i==self.len()-1{n.to_string()} else if i==self.len()-2 {format!("{}*{}", n, x)}
            else{format!("{}*{}^{}", n, x,self.len()-i-1)};
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
        if g.is_zero(){return [g.clone(),h.clone()].to_vec()}
            let zero=g.coeffs[0].clone().zero();
            let mut rem=g.clone();
            let mut q=g.zero();
        while rem.deg()>=h.deg()&& !rem.is_zero(){
            let lcr=rem.coeffs[0].clone();
            let lcq=h.coeffs[0].inverse();
            let new_len=rem.len()-h.len()+1;
            let mut coeffs:Vec<F> =vec![zero.clone();new_len];
            coeffs[0]=lcr*lcq;
            let s=Poly::new_from_coeffs(&coeffs);
            q=&q+&s;
            rem=&rem-&(&s*&h);
        }
        [q,rem].to_vec()

}
}
impl <F:Field> Poly<F>{
/// # Example
/// ```
/// use num_bigint::BigInt;
/// use algebra::poly::Poly;
/// use algebra::intmod::{Mod,PrimeField};
/// use algebra::field::Field;
/// let prime_base=BigInt::from(13);
/// let z13=PrimeField(Some(BigInt::from(13)));
/// let p1 = Poly::new_from_coeffs(&[z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3))]);
/// let alpha= z13.new(BigInt::from(3));
/// let value = z13.new(BigInt::from(5));
/// assert_eq!(value,p1.evaluate(&alpha));
/// ```
/// 
pub fn evaluate(&self,alpha:&F)->F {
    let mut value = self.coeffs[0].zero();
    for i in 0..self.len(){
        value=value*alpha.clone()+self.coeffs[i].clone();
    }
    value
}
pub fn normal_poly(&mut self)->Poly<F> {
    let c=self.coeffs.last().unwrap().inverse();
    for i in 0..self.len(){self.coeffs[i]=self.coeffs[i].clone()*&c}
    Poly::new_from_coeffs(&mut self.coeffs)
    
}
pub fn multiple(&mut self, alpha:&F)->Poly<F>{
    for i in 0..self.len(){
    self.coeffs[i]=self.coeffs[i].clone()*alpha;}
    Poly::new_from_coeffs(&mut self.coeffs)
}
pub fn gcdext(g:&Poly<F>,h:&Poly<F>)->Vec<Poly<F>>{
    let mut u = g.one(); let mut d=g.clone();
    let mut v1=h.zero(); let mut v3=h.clone();
    while !v3.is_zero() {
        let div_rem= Poly::div_rem(&d,&v3);
        let t= &u-&(&v1*&div_rem[0]);
        u=v1;
        d=v3;
        v1=t;
        v3=div_rem[1].clone();
    }
    let v=&(&d-&(g*&u))/h;
    [u,v,d].to_vec()
    }
pub fn is_coprime(g:&Poly<F>,h:&Poly<F>)->bool{
    Poly::gcdext(g, h)[2].is_one()
}
}