
use std::vec;
use algebra::poly;
use algebra::polymod::Modulus;
use algebra::{integers::IntUtilities, poly::Poly};
use num_bigint::{BigUint,BigInt,RandomBits, ToBigInt};
use algebra::intmod::{Mod, PrimeField};
use num_traits::{One, Zero};
use rand::Rng;
use num_bigint::RandBigInt;
use std::io;


    fn main() {
let z13=PrimeField(Some(BigInt::from(13)));
let p1 = Poly::new_from_coeffs(&[z13.new(BigInt::from(9)), z13.new(BigInt::from(0)),z13.new(BigInt::from(1))]);
let p2 = Poly::new_from_coeffs(&[z13.new(BigInt::from(1)), z13.new(BigInt::from(1))]);
let p3 = Poly::new_from_coeffs(&[z13.new(BigInt::from(2)),z13.new(BigInt::from(1)), z13.new(BigInt::from(1))]);
let modulus=Modulus(p1.clone());
let polymod1=modulus.new(p2.clone());
let polymod2=modulus.new(p3.clone());
println!("a poly mod {} with coeffs {:?}",polymod1, polymod1.poly.coeffs);
println!("a poly mod {} with coeffs {:?}",polymod2,polymod2.poly.coeffs);
println!("sum mod is {}",polymod1.inverse());



        /* PROMPT CMD
        
        println!("Please type something, or x to escape:");
        let mut input_string = String::new();
    
        while input_string.trim() != "x" {
            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            println!("You wrote {}", input_string);
        }
        println!("See you later!"); */
    }
    