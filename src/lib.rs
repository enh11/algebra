//! This library contains utilities for integer number
//! Good references for this algorithm is [H.Cohen A course in computational number theory] 
//! and [H. Cohen Handbook ok elliptic and hyperelliptic curves cryptography]
//!

pub mod integers;
pub mod intmod;
pub mod poly;
pub mod field;
pub mod polymod;

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use crate::intmod::PrimeField;
    use crate::{integers::IntUtilities, intmod::{Mod}};
    use crate::poly::Poly;
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
    assert_eq!(BigInt::even_part(a),expected_value);
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
    let a=Mod::new(BigInt::from(8),PrimeField(Some(BigInt::from(11))));
    let b=Mod::new(BigInt::from(4),PrimeField(Some(BigInt::from(13))));
    let expected_mod=Some(Mod::new(BigInt::from(30),PrimeField(Some(BigInt::from(143)))));
    assert_eq!(expected_mod,Mod::chinese(&[a,b].to_vec()));

}
#[test]
fn test_gcdext_polynomial(){
    let z13=PrimeField(Some(BigInt::from(13)));
    let p1 = Poly::new_from_coeffs(&[z13.new(BigInt::from(9)), z13.new(BigInt::from(0)),z13.new(BigInt::from(1))]);
    let p2 = Poly::new_from_coeffs(&[z13.new(BigInt::from(0)), z13.new(BigInt::from(1))]);
    let expected_pol_d=Poly::new_from_coeffs(&[z13.new(BigInt::from(9))]);
    let expected_pol_u=Poly::new_from_coeffs(&[z13.new(BigInt::from(1))]);
    let expected_pol_v=Poly::new_from_coeffs(&[z13.new(BigInt::from(0)),z13.new(BigInt::from(12))]);
let bez=Poly::gcdext(&p1,&p2);
assert_eq!(bez,[expected_pol_u,expected_pol_v,expected_pol_d]);
}
#[test]
fn test_coprime_polynomial(){
    let z13=PrimeField(Some(BigInt::from(13)));
    let p1 = Poly::new_from_coeffs(&[z13.new(BigInt::from(9)), z13.new(BigInt::from(0)),z13.new(BigInt::from(1))]);
    let p2 = Poly::new_from_coeffs(&[z13.new(BigInt::from(0)), z13.new(BigInt::from(1))]);
    let expected_pol_d=Poly::new_from_coeffs(&[z13.new(BigInt::from(9))]);
assert_ne!(Poly::is_coprime(&p1,&p2),true);

}
}