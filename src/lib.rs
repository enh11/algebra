//! This library contains utilities for integer number
//! Good references for this algorithm is [H.Cohen A course in computational number theory] 
//! and [H. Cohen Handbook ok elliptic and hyperelliptic curves cryptography]
//!
#[macro_use]
pub mod integers;
pub mod intmod;
pub mod poly;
pub mod field;
pub mod polymod;

#[cfg(test)]
mod tests {
 
    use num_bigint::BigInt;
    use crate::intmod::PrimeField;
    use crate::{integers::IntUtilities, intmod::Mod};
    use crate::poly::Poly;
    use crate::poly;
    use crate::intmod::MathError::QuadraticNonResidueModP;
#[test]
fn test_gcd() {
    let a=BigInt::from(60u8);
    let b=BigInt::from(24u8);
    let expected_value=BigInt::from(12u8);
    assert_eq!(BigInt::gcd(a,b),expected_value);
}
#[test]
fn test_even_part() {
    let a=BigInt::from(24u8);
    let expected_value=(BigInt::from(8u8),3u64);
    assert_eq!(BigInt::even_part(&a),expected_value);
}
#[test]
fn test_gcdext() {
    let a=BigInt::from(24u32);
    let b=BigInt::from(60u32);
    let bezout=BigInt::gcdext(a.clone(), b.clone());
    
    assert_eq!(&bezout[0]*a+&bezout[1]*b,bezout[2]);
}
#[test]
fn test_chinese() {
    let a=Mod::new(BigInt::from(8),PrimeField(BigInt::from(11)));
    let b=Mod::new(BigInt::from(4),PrimeField(BigInt::from(13)));
    let expected_mod=Some(Mod::new(BigInt::from(30),PrimeField(BigInt::from(143))));
    assert_eq!(expected_mod,Mod::chinese(&[a,b].to_vec()));

}
#[test]
fn test_gcdext_polynomial(){
    let z13=PrimeField(BigInt::from(13));
    let p1 = Poly::new_from_coeffs(&mut [z13.new(BigInt::from(1)), z13.new(BigInt::from(0)),z13.new(BigInt::from(9))]);
    let p2 = Poly::new_from_coeffs(&mut [z13.new(BigInt::from(1)), z13.new(BigInt::from(0))]);
    let expected_pol_d=Poly::new_from_coeffs(&mut [z13.new(BigInt::from(9))]);
    let expected_pol_u=Poly::new_from_coeffs(&mut [z13.new(BigInt::from(1))]);
    let expected_pol_v=Poly::new_from_coeffs(&mut [z13.new(BigInt::from(12)),z13.new(BigInt::from(0))]);
let bez=Poly::gcdext(&p1,&p2);
assert_eq!(bez,[expected_pol_u,expected_pol_v,expected_pol_d]);
}
#[test]
fn text_poly_ops(){

let z13=PrimeField(BigInt::from(13));
let p1 = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3)));
let p2 = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(2)));
let add = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(7)),z13.new(BigInt::from(5)));
let sub = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(10)),z13.new(BigInt::from(1)));
let prod = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(12)),z13.new(BigInt::from(6)),z13.new(BigInt::from(6)));
let quoz= poly!(z13.new(BigInt::from(8)),z13.new(BigInt::from(5)));
let rem = poly!(z13.new(BigInt::from(6)));
assert_eq!(add,&p1+&p2);
assert_eq!(sub,&p1-&p2);
assert_eq!(prod,&p1*&p2);
assert_eq!(quoz,&p1/&p2);
assert_eq!(rem,&p1%&p2);

}
#[test]
fn test_coprime_polynomial(){
    let z13=PrimeField(BigInt::from(13));
    let p1 = Poly::new_from_coeffs(&mut [z13.new(BigInt::from(1)), z13.new(BigInt::from(0)),z13.new(BigInt::from(9))]);
    let p2 = Poly::new_from_coeffs(&mut [z13.new(BigInt::from(1)), z13.new(BigInt::from(0))]);
assert_ne!(Poly::is_coprime(&p1,&p2),true);

}
#[test]
fn test_sqrt_mod_prime(){

let z17=PrimeField(BigInt::from(17));
let mut n=z17.new(BigInt::from(15));
let mut m=z17.new(BigInt::from(3));
let expected_sqrt=z17.new(BigInt::from(7));
assert_eq!(expected_sqrt,n.sqrt_mod_prime());
assert_eq!(Err(QuadraticNonResidueModP),m.check_sqrt_mod_prime());


}
}