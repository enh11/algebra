
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


fn main() {
    let z13=PrimeField(BigInt::from(13));

    let index0=MultiIndex::new(&vec![2,1]);
    let index1=MultiIndex::new(&vec![1,2]);
    let index2=MultiIndex::new(&vec![0,2]);
    let index3=MultiIndex::new(&vec![0]);
    let index4=MultiIndex::new(&vec![1,1]);
    let index5=MultiIndex::new(&vec![1]);
    let index6=MultiIndex::new(&vec![0,1]);
    
    let term0=Terms::new(z13.one(), index0);//x_0^2*x_1
    let term1=Terms::new(z13.one(), index1);//x_0*x_1^2
    let term2=Terms::new(z13.one(), index2);//x_1^2
    let term3=Terms::new(z13.one().neg(), index3);//-1
    let term4=Terms::new(z13.one(), index4); //x_0x_1
    let term5= Terms::new(z13.one(), index5);
    let term6= Terms::new(z13.one(), index6);

let mut m1= MultivariatePoly::new(vec![term0.clone(),term1.clone(),term2.clone()]);//x_0^2*x_1+x_0*x_1^2+x_1^2
let mut m2= MultivariatePoly::new(vec![term4.clone(),term3.clone()]);//x_0*x_1-1
let quot_rem = &m1.div_rem(&mut m2);
println!("quotient is {}, rem is {}",quot_rem.0,quot_rem.1);
let proof=&(&m2*&quot_rem.0);
println!("proof is {}",proof);
let proof =proof+&quot_rem.1;
println!("proof is {}\nm1 was {m1}",proof);
/* 
let m3= MultivariatePoly::new(vec![term2.clone(),term3.clone()]);//x_1^2-1
let multi_div= m1.multi_division_reminder(vec![m2.clone(),m3.clone()]);
println!("{m1} divided by {m2} and {m3}");
for item in &multi_div.0 {
    println!("quotient is {}",item);
}

println!("reminder is {}",multi_div.1);

let expected_q0=MultivariatePoly::new(vec![term5.clone(),term6.clone()]);
let expected_q1=MultivariatePoly::new(vec![term3.neg()]);
let expected_reminder = MultivariatePoly::new(vec![term5,term6,term3.neg()]);
assert_eq!(multi_div.0[0],expected_q0);

assert_eq!(multi_div.0[1],expected_q1);
assert_eq!(multi_div.1,expected_reminder);
println!("everything ok");

 */
/*  
let q=&m1/&m2;
let r = &m1%&m2;
println!("quozient is {}, reminder is {}",q,r);
let proof=&m2*&q+r;
assert_eq!(m1,proof); 
*/
/* 
println!(" {} {:?} {}",term4.clone(),term4.cmp(&term4.neg().clone()),term4.neg());
let mut m3= MultivariatePoly::new(vec![term4.clone(),term3.clone()]);
let mut m4= MultivariatePoly::new(vec![term4.clone(),term3.neg().clone()]);
println!("[{}]/[{}]={}",m3.clone(),m4.clone(),&mut m3*&mut m4);
println!("m4 is still here {}",m4); */

/* assert_eq!(m1,proof);
 */
//println!("gcd is {}",Poly::gcdext(&p1, &p2)[0]);
/* let polymod1=modulus.new(p1.clone());
let polymod2=modulus.new(p2.clone());
println!("a poly mod {} with coeffs {:?}",polymod1, polymod1.poly.coeffs);
println!("a poly mod {} with coeffs {:?}",polymod2,polymod2.poly.coeffs); */



        /* PROMPT CMD
        
        println!("Please type something, or "quit" to escape:");
        let mut input_string = String::new();
    
        while input_string.trim() != "quit" {
            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            println!("You wrote {}", input_string);
        }
        println!("See you later!"); */
    }
    