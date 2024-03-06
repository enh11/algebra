
use std::ops::Neg;
use std::vec;
use algebra::field::Field;
use algebra::multivariatepoly::{Monomial, MultiIndex, MultivariatePoly, Terms};
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
    let mut index1=MultiIndex::new(&vec![1,0]);
    let mut index2=MultiIndex::new(&vec![1,1,4]);
    println!("index2.weight is {}",index2.weight());
    let mut index0=MultiIndex::new(&vec![3,1,2]);
let z13=PrimeField(BigInt::from(13));
let mut term0=Terms::new(z13.random(), index0);
println!("term0 is {}. is zero is {}",term0,term0.is_zero());
let mut term1=Terms::new(z13.random(), index1);
println!("term1 is {}. is zero is {}",term1,term1.is_zero());
println!("mul is {}",&mut term0/&mut term1);


let mut mono1 = Monomial::new_from_multi_index(vec![2,1,4]);
let mut mono2=Monomial::new_from_multi_index(vec![1,0,1]);
let mono3=Monomial::new_from_multi_index(vec![1,0,1]);
let mut mono4=Monomial::new_from_multi_index(vec![1,5,2]);
println!("{} divided by {} is {}",mono1.clone(),mono2.clone(),&mut mono1/&mut mono2);
let multivariate1=MultivariatePoly::new(vec![(z13.one(),mono1.clone()),(z13.one(),mono3.clone())]);
let multivariate2=MultivariatePoly::new(vec![(z13.new(BigInt::from(3)),mono3.clone())]);

println!("({})*({}) = {}",multivariate1.clone(),multivariate2.clone(),multivariate1*multivariate2);


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
    