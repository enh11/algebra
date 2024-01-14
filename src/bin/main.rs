
use std::vec;
use algebra::poly;
use algebra::{integers::IntUtilities, poly::Poly};
use num_bigint::{BigUint,BigInt,RandomBits, ToBigInt};
use algebra::intmod::{Mod, PrimeField};
use num_traits::{One, Zero};
use rand::Rng;
use num_bigint::RandBigInt;


fn main() {
let z13=PrimeField(Some(BigInt::from(13)));
let p1 = Poly::new_from_coeffs(&[z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3))]);
let p2 = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(2)));
println!("poly1 {}",p1);
println!("poly2 {}",p2);
let q=&p1/&p2;
let r = &p1%&p2;
println!("quotient is {} and reminder is {}",q,r);



    

}