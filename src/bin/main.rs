
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
let modd=Mod::new(BigInt::from(13), z13.clone());
println!("{:?}",modd);
let p1 = Poly::new_from_coeffs(&[z13.new(BigInt::from(9)), z13.new(BigInt::from(0)),z13.new(BigInt::from(1))]);
let p2 = poly!(z13.new(BigInt::from(0)), z13.new(BigInt::from(1)));
println!("poly1 {}",p1);
println!("poly2 {}",p2);
let bez=Poly::gcdext(&p1,&p2);
println!("bezout: u={},v={},d={}",bez[0],bez[1],bez[2]);
let check=&(&p1*&bez[0])+&(&p2*&bez[1]);
println!("check {}",check);


    

}