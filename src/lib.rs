//! This library contains utilities for integer number
//! Good references for this algorithm is [H.Cohen A course in computational number theory] 
//! and [H. Cohen Handbook ok elliptic and hyperelliptic curves cryptography]
//!

pub mod integers;
pub mod intmod;
pub mod poly;
pub mod field;


#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use crate::{integers::IntUtilities, intmod::{Mod, PrimeField}};

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
}