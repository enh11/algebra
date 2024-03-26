use core::fmt;
use std::ops::{Add, Sub,Neg,Mul};
use crate::field::Field;

use super::poly::Poly;
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
    pub fn is_zero(&self)->bool{
        self.poly.is_zero()
    }
    pub fn is_one(&self)->bool{
        self.poly.is_one()
    }
    pub fn one(&self)->Self{
        Self::new(Poly::one(&self.poly), self.modulus.clone())
    }
    pub fn inverse(&self)->Self{
        let bez=Poly::gcdext(&self.poly, &self.modulus.0);
        println!("bez[2] is {}",bez[2]);
        if !bez[2].is_constant() {panic!("{} is not invertible mod {}",self.poly,self.modulus.0);}
        PolyMod::new(bez[0].clone().multiple(&bez[2].coeffs[0].inverse()),self.modulus.clone())
    }
/// # Example
/// ```
/// #[macro_use] extern crate algebra;
/// use num_bigint::BigInt;
/// use algebra::univariate::polymod::{PolyMod,Modulus};
/// use algebra::univariate::poly::Poly;
/// use algebra::intmod::{Mod,PrimeField};
/// use algebra::field::Field;
/// let prime_base=BigInt::from(13);
/// let z13=PrimeField(BigInt::from(13));
/// let modulus1=Modulus(Poly::new_from_coeffs(&[z13.one(), z13.one(), z13.one()]));
/// let modulus2 = Modulus(Poly::new_from_coeffs(&[z13.one(), z13.one()]));
/// let p1 = modulus1.new(Poly::new_from_coeffs(&[z13.one(), z13.one()]));
/// let p2=modulus2.new(Poly::new_from_coeffs(&[z13.one(), z13.new(BigInt::from(2))]));
/// let chinese = PolyMod::chinese(vec![&p1,&p2]).unwrap();
/// let chinese_proof1=modulus1.new(chinese.clone().poly);
/// let chinese_proof2=modulus2.new(chinese.clone().poly);
/// 
/// assert_eq!(chinese_proof1,p1);
/// assert_eq!(chinese_proof2,p2);
/// 
/// ```
    pub fn chinese(moduli:Vec<&PolyMod<F>>)->Option<Self> {
        let mut m=moduli[0].modulus.0.clone();
        let mut x=moduli[0].poly.clone();
        
        for i in 1..moduli.len() {
            let uvd=Poly::gcdext(&moduli[i].modulus.0, &m);
            if uvd[2]!=x.one() {return None;} 
            x=(&uvd[0]*&moduli[i].modulus.0)*x+&(&uvd[1]*&moduli[i].poly)*&m;
            m=&m*&moduli[i].modulus.0;
            x=&x%&m;               
        }
            Some(PolyMod::new(x, Modulus(m)))

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
impl<'a,'b,F:Field> Add<&'b PolyMod<F>> for PolyMod<F>{
    type Output = Self;
    fn add(self, rhs: &'b PolyMod<F>) -> Self::Output {
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
    fn sub(self, rhs: PolyMod<F>) -> Self::Output {
        if self.modulus==rhs.modulus{
            PolyMod::new(&self.poly-&rhs.poly,self.modulus)
        }
            else {panic!("cannot add different modulus")}
}
}
impl<'a,'b,F:Field> Sub<&'b PolyMod<F>> for PolyMod<F> {
    type Output = Self;    
    fn sub(self, rhs: &'b PolyMod<F>) -> Self::Output {
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
impl<'a,'b,F:Field> Mul<&'b PolyMod<F>> for PolyMod<F> {
    type Output = Self;    
    fn mul(self, rhs: &'b Self) -> Self::Output {
        if self.modulus==rhs.modulus{
            let mut poly=&self.poly*&rhs.poly;
            poly=&poly%&self.modulus.0;
            PolyMod::new(poly,self.modulus)
        }
            else {panic!("cannot add different modulus")}
}
}