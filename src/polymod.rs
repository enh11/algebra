use core::fmt;
use std::ops::{Add, Sub,Neg,Mul};

use num_traits::Zero;

use crate::{poly::{self, Poly}, field::Field};
#[derive(Debug,PartialEq,Eq, PartialOrd, Ord,Clone)]
pub struct Modulus<F>(pub Poly<F>);
impl <F: Field>Modulus<F> {
    pub fn new(&self,g:Poly<F>)->PolyMod<F>{
        let poly=&g%&self.0;
        PolyMod::new(poly, self.clone())
    }
    pub fn one(self)->PolyMod<F>{
        PolyMod::new(self.0.one(), self)

    }
    pub fn zero(self)->PolyMod<F>{
        PolyMod::new(self.0.zero(), self)

    }
    
}
#[derive(Debug,PartialEq,Eq, PartialOrd, Ord,Clone)]
pub struct PolyMod<F> {
    pub poly:Poly<F>,
    pub modulus:Modulus<F>
}
impl<F:Field>PolyMod<F>{
    pub fn new(poly:Poly<F>,modulus:Modulus<F>)->Self{
        PolyMod{poly,modulus}
    }
    pub fn zero(&self)->Self{
        Self::new(Poly::zero(&self.poly),self.modulus.clone())
    }
    pub fn one(&self)->Self{
        Self::new(Poly::one(&self.poly), self.modulus.clone())
    }
    pub fn inverse(&self)->Self{
        //TO BE FIXED!! gcd is Mod(n,m), so we need to check if is invertible!
        let bez=Poly::gcdext(&self.poly, &self.modulus.0);
        println!("bez[2] is {}",bez[2]);
        if bez[2]>self.poly.one() {panic!("{} is not invertible mod {}",self.poly,self.modulus.0);}
        PolyMod::new(bez[0].clone(),self.modulus.clone())
    }
}
impl<F:Field> fmt::Display for PolyMod<F> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "PolyMod({},{})", Poly::print_poly(&self.poly,"x"),Poly::print_poly(&self.modulus.0, "x"))
    }
}
impl<F:Field> Add<PolyMod<F>> for PolyMod<F> {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        if self.modulus==rhs.modulus{
                let modulus=&self.modulus.0;
                let mut sum = &self.poly+&rhs.poly;
                if sum>=*modulus{
                    sum = &sum-&modulus;
                    PolyMod::new(sum, self.modulus.clone())}
                    else {PolyMod::new(sum, self.modulus)}}
                else {panic!("cannot add different modulus")}
}
}
impl <F:Field> Neg for PolyMod<F>{
    type Output = Self;
    fn neg(self) -> Self::Output {

            PolyMod::new(&self.modulus.0-&self.poly,self.modulus.clone())
    }
}
impl<F:Field> Sub<PolyMod<F>> for PolyMod<F> {
    type Output = Self;    
    fn sub(self, rhs: Self) -> Self::Output {
        if self.modulus==rhs.modulus{
            PolyMod::new(&self.poly-&rhs.poly,self.modulus)
        }
            else {panic!("cannot add different modulus")}
}
}
impl<F:Field> Mul<PolyMod<F>> for PolyMod<F> {
    type Output = Self;    
    fn mul(self, rhs: Self) -> Self::Output {
        if self.modulus==rhs.modulus{
            let mut poly=&self.poly*&rhs.poly;
            poly=&poly%&self.modulus.0;
            PolyMod::new(poly,self.modulus)
        }
            else {panic!("cannot add different modulus")}
}
}