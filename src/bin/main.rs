
use std::vec;
use algebra::multivariatepoly::{Monomial, MultivariatePoly};
use algebra::poly;
use algebra::polymod::{Modulus, PolyMod};
use algebra::{integers::IntUtilities, poly::Poly};
use num_bigint::{BigUint,BigInt,RandomBits, ToBigInt};
use algebra::intmod::{Mod, PrimeField};
use num_traits::{One, Zero};
use rand::Rng;
use num_bigint::RandBigInt;
use std::io;


fn main() {

let z13=PrimeField(BigInt::from(13));
let mono = Monomial::new_from_multi_index(vec![2,0,0,1,4,0]);
println!("monomial is {}",mono);
let mono2=Monomial::new_from_multi_index(vec![4,5,5,6,1]);
println!("monomial 2 is {}",mono2);

let multivariate=MultivariatePoly::new(vec![(z13.one(),mono),(z13.new(BigInt::from(4)),mono2)]);
println!("multivariate is {}",multivariate);

let p1 = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3)));
let p2 = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(2)));
let p3 = poly!(z13.new(BigInt::from(4)),z13.new(BigInt::from(0)),z13.new(BigInt::from(3)),z13.new(BigInt::from(1)));
let p4 = poly!(z13.new(BigInt::from(3)),z13.new(BigInt::from(4)),z13.new(BigInt::from(0)),z13.new(BigInt::from(3)),z13.new(BigInt::from(1)));

let f=Modulus(p3);
let g=Modulus(p4);
println!("modulus is {}",f.0);
let mut  pmod1=f.new(p1);
let mut pmod2=g.new(p2);


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
    