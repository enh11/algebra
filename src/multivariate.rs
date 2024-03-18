//! This library contains utilities for Multivariate polynomial.
//! Good references for algorithms below is 
//! [https://scholar.google.it/scholar_url?url=https://www.mdpi.com/2227-7390/7/5/441/pdf&hl=it&sa=X&ei=jRjnZeHFOZWty9YP9Me4iAs&scisig=AFWwaea_Q77frjP2J8Auw8F8Tfl-&oi=scholarr]
//! [David A. Cox John Little Donal O'Shea Ideals,Varieties, and Algorithms]

pub mod multivariatepoly;
pub mod multiindex;
pub mod terms;

#[cfg(test)]
mod tests {
    use std::ops::Neg;
    use std::vec;
    use num_bigint::BigInt;
    use crate::intmod::PrimeField;
    use crate::multivariate::multiindex::MultiIndex;
    use crate::multivariate::multivariatepoly::MultivariatePoly;
    use crate::multivariate::terms::Terms;
#[test]
fn test_multivariate_division() {
let z13=PrimeField(BigInt::from(13));
    
let index0=MultiIndex::new(&vec![2,1]);
let index1=MultiIndex::new(&vec![1,2]);
let index2=MultiIndex::new(&vec![1,1]);
let index3=MultiIndex::new(&vec![0]);
let index4=MultiIndex::new(&vec![0,2]);

let term0=Terms::new(z13.one(), index0);
let term1=Terms::new(z13.one(), index1);
let term2=Terms::new(z13.one(), index4);
let term3=Terms::new(z13.one().neg(), index3);
let term4=Terms::new(z13.one(), index2.clone());

let m1= MultivariatePoly::new(vec![term0,term1,term2]);
let mut m2= MultivariatePoly::new(vec![term3,term4]);
let mut q=&m1/&mut m2;

let r = &m1%&mut m2;
let proof=&mut m2*&mut q+r;
assert_eq!(m1,proof);

}
#[test]
fn test_multi_division() {
    /* In this test we test for multidivision.
    We want to stress that the quotients output depends
    on the ordering of the s-tuple of polynomials in the
    divisors input ( f1, . . . , fs) */
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
let term5= Terms::new(z13.one(), index5.clone());//x_0
let term6= Terms::new(z13.one(), index6);//x_1
let term7= Terms::new(z13.new(BigInt::from(2)), index5);//2*x_0

let mut m1= MultivariatePoly::new(vec![term0.clone(),term1.clone(),term2.clone()]);//x_0^2*x_1+x_0*x_1^2+x_1^2
let m2= MultivariatePoly::new(vec![term4.clone(),term3.clone()]);//x_0*x_1-1
let m3= MultivariatePoly::new(vec![term2.clone(),term3.clone()]);//x_1^2-1
let multi_div= m1.multi_division_reminder(vec![m2.clone(),m3.clone()]);

let expected_q0=MultivariatePoly::new(vec![term5.clone(),term6.clone()]);
let expected_q1=MultivariatePoly::new(vec![term3.neg()]);
let expected_reminder = MultivariatePoly::new(vec![term5.clone(),term6,term3.neg()]);

assert_eq!(multi_div.0[0],expected_q0);
assert_eq!(multi_div.0[1],expected_q1);
assert_eq!(multi_div.1,expected_reminder);

let multi_div= m1.multi_division_reminder(vec![m3,m2]);
let expected_q0=MultivariatePoly::new(vec![term5.clone(),term3.neg()]);
let expected_q1=MultivariatePoly::new(vec![term5]);
let expected_reminder = MultivariatePoly::new(vec![term7,term3.neg()]);

assert_eq!(multi_div.0[0],expected_q0);
assert_eq!(multi_div.0[1],expected_q1);
assert_eq!(multi_div.1,expected_reminder);

}

}
