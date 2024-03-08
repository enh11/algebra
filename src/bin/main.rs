
use std::ops::Neg;
use std::vec;
use algebra::field::Field;
use algebra::multivariatepoly::{MultiIndex, MultivariatePoly, Terms};
use algebra::poly;
use algebra::polymod::{Modulus, PolyMod};
use algebra::{integers::IntUtilities, poly::Poly};
use num_bigint::{BigUint,BigInt,RandomBits, ToBigInt};
use algebra::intmod::{Mod, PrimeField};
use num_traits::{One, Zero};
use rand::Rng;
use num_bigint::RandBigInt;
use std::io;
use std::collections::BinaryHeap;


fn main() {
    let mut index0=MultiIndex::new(&vec![1,1]);
    let mut index1=MultiIndex::new(&vec![1,0]);
    let mut index2=MultiIndex::new(&vec![2,2]);
    let mut index3=MultiIndex::new(&vec![2,0]);
    println!("is divisible index3 by index1 {}",&mut index1.is_subtractable_by(&mut index3));
    let mut index4=MultiIndex::new(&vec![3,0,2]);
let z13=PrimeField(BigInt::from(13));
let term0=Terms::new(z13.one(), index0.clone());
let term12=Terms::new(z13.one(), index1.clone());
let term21=Terms::new(z13.one().neg(), index1.clone());
let  m1= MultivariatePoly::new(vec![term0.clone(),term12.clone()]);
let  m2= MultivariatePoly::new(vec![term0.clone(),term21.clone()]);

let try_prod = m1*m2;
println!("try_sum {}",try_prod);
println!("term0 is {}. is zero is {}",term0,term0.is_zero());
let term1=Terms::new(z13.one().neg(), index1.clone());
println!("term1 is {}. is zero is {}",term1,term1.is_zero());
let term2=Terms::new(z13.one(), index2);
println!("term2 is {}. is zero is {}",term2,term2.is_zero());
let term3=Terms::new(z13.new(BigInt::from(12)), index3);
println!("term3 is {}. is zero is {}",term3,term3.is_zero());
let term4=Terms::new(z13.random(), index4);
println!("term4 is {}. is zero is {}",term4,term4.is_zero());

let mut multivariate1=MultivariatePoly::new(vec![term3.clone(),term2.clone()]);
let mut multivariate2=MultivariatePoly::new(vec![term0.clone(),term1.clone()]);
println!("mult1 ={},multi2 = {}",multivariate1.clone(),multivariate2.clone());
println!("[{}]/[{}]={}",multivariate1.clone(),multivariate2.clone(),&mut multivariate1/&mut multivariate2);

//println!("gcd is {}",Poly::gcdext(&p1, &p2)[0]);
/* let polymod1=modulus.new(p1.clone());
let polymod2=modulus.new(p2.clone());
println!("a poly mod {} with coeffs {:?}",polymod1, polymod1.poly.coeffs);
println!("a poly mod {} with coeffs {:?}",polymod2,polymod2.poly.coeffs); */



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
    