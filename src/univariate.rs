#[macro_use]
pub mod poly;
pub mod polymod;

#[cfg(test)]
mod tests {
use std::ops::Neg;

use num_bigint::BigInt;
use crate::{intmod::PrimeField, univariate::poly::Poly};

use super::polymod::{Modulus, PolyMod};
#[test]
fn test_add_poly() {
let z13=PrimeField(BigInt::from(13));
let p1 = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3)));
let p2 = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(2)));
println!("poly p1 is {}",p1);
println!("poly p2 is {}",p2);
println!("poly sum is {}",&p1+&p2);
println!("poly sub is {}",&p1-&p2);
println!("poly mul is {}",&p1*&p2);
println!("poly quotient is {}",&p1/&p2);
print!("poly reminder is {}",&p1%&p2);
let expected_add = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(7)),z13.new(BigInt::from(5)));
let expected_sub = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(10)),z13.new(BigInt::from(1)));
let expected_mul = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(12)),z13.new(BigInt::from(6)),z13.new(BigInt::from(6)));
let expected_quot=poly!(z13.new(BigInt::from(8)),z13.new(BigInt::from(5)));
let expected_rem=poly!(z13.new(BigInt::from(6)));

    assert_eq!(expected_add,&p1+&p2);
    assert_eq!(expected_sub,&p1-&p2);
    assert_eq!(expected_mul,&p1*&p2);
    assert_eq!(expected_quot,&p1/&p2);
    assert_eq!(expected_rem,&p1%&p2);

    }
#[test]
fn gcdext_univariate() {
    let z13=PrimeField(BigInt::from(13));
    let p1 = Poly::new_from_coeffs(&[z13.new(BigInt::from(1)), z13.zero(), z13.new(BigInt::from(10)),z13.new(BigInt::from(2))]);
    let p2 = Poly::new_from_coeffs(&[z13.new(BigInt::from(1)),z13.zero(),z13.one().neg()]);
    let gcd = Poly::gcdext(&p1,&p2);

    let expected_gcd= Poly::new_from_coeffs(&[z13.new(BigInt::from(11)),z13.new(BigInt::from(2))]);
    assert_eq!(gcd[2],expected_gcd);
}
#[test]
fn test_bezout_univariate(){
    let z13=PrimeField(BigInt::from(13));
    let p1 = Poly::new_from_coeffs(&[z13.new(BigInt::from(1)), z13.zero(), z13.new(BigInt::from(10)),z13.new(BigInt::from(2))]);
    let p2 = Poly::new_from_coeffs(&[z13.new(BigInt::from(1)),z13.zero(),z13.one().neg()]);
    let bezout = Poly::gcdext(&p1,&p2);
    let gcd=&p1*&bezout[0]+&p2*&bezout[1];
    assert_eq!(gcd,bezout[2])
}
#[test]
fn test_chinese_univariate() {
    let z13=PrimeField(BigInt::from(13));
    let modulus1=Modulus(Poly::new_from_coeffs(&[z13.one(), z13.one(), z13.one()]));
    let modulus2 = Modulus(Poly::new_from_coeffs(&[z13.one(), z13.one()]));
    
    let p1 = modulus1.new(Poly::new_from_coeffs(&[z13.one(), z13.one()]));
    let p2=modulus2.new(Poly::new_from_coeffs(&[z13.one(), z13.new(BigInt::from(2))]));
   
    let chinese = PolyMod::chinese(vec![&p1,&p2]).unwrap();
    let chinese_proof1=modulus1.new(chinese.clone().poly);
    let chinese_proof2=modulus2.new(chinese.clone().poly);

    assert_eq!(chinese_proof1,p1);
    assert_eq!(chinese_proof2,p2);


    
    



}
}