
use std::ops::Neg;
use std::vec;
use algebra::multivariate::multiindex::MultiIndex;
use algebra::multivariate::terms::Terms;
use algebra::multivariate::multivariatepoly::{MultivariatePoly};
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
    let mut index0=MultiIndex::new(&vec![2,1]);
    let mut index1=MultiIndex::new(&vec![1,2]);
    let mut index2=MultiIndex::new(&vec![1,1]);
    let mut index4=MultiIndex::new(&vec![0,2]);

    let mut index3=MultiIndex::new(&vec![0]);
    println!("ordering {:?}<{:?}: {:?}", index3.clone(),index2.clone(),index3.cmp(&index2));
let z13=PrimeField(BigInt::from(13));
let term0=Terms::new(z13.one(), index0.clone());
let term1=Terms::new(z13.one(), index1.clone());
let term2=Terms::new(z13.one(), index4.clone());
let term4=Terms::new(z13.one(), index2.clone()); //x_0x_1

let term3=Terms::new(z13.one(), index3);//-1

let mut m1= MultivariatePoly::new(vec![term0.clone(),term1.clone(),term2.clone()]);
let mut  m2= MultivariatePoly::new(vec![term4.clone(),term3.clone()]);

let q=&mut m1/&mut m2;
println!("[{}]/[{}]={}",m1.clone(),m2.clone(),q);
let proof=q*m2;
println!("proof is {proof}");
println!("m1 is {m1}");

println!(" {} {:?} {}",term4.clone(),term4.cmp(&term4.neg().clone()),term4.neg());
let mut m3= MultivariatePoly::new(vec![term4.clone(),term3.clone()]);
let mut m4= MultivariatePoly::new(vec![term4.clone(),term3.neg().clone()]);
println!("[{}]/[{}]={}",m3.clone(),m4.clone(),&mut m3*&mut m4);
println!("m4 is still here {}",m4);

/* assert_eq!(m1,proof);
 */
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
    