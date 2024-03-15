pub mod multivariatepoly;
pub mod multiindex;
pub mod terms;

#[cfg(test)]
mod tests {
    use std::ops::{Mul, Neg};
    use std::vec;
    use num_bigint::BigInt;
    use crate::intmod::PrimeField;
    use crate::multivariate::multiindex::MultiIndex;
    use crate::multivariate::multivariatepoly::MultivariatePoly;
    use crate::multivariate::terms::Terms;
#[test]
fn test_multivariate_division() {
    let index0=MultiIndex::new(&vec![2,1]);
    let index1=MultiIndex::new(&vec![1,2]);
    let index2=MultiIndex::new(&vec![1,1]);
    let index3=MultiIndex::new(&vec![0]);
    let index4=MultiIndex::new(&vec![0,2]);
let z13=PrimeField(BigInt::from(13));
let term0=Terms::new(z13.one(), index0);
let term1=Terms::new(z13.one(), index1);
let term2=Terms::new(z13.one(), index4);
let term3=Terms::new(z13.one().neg(), index3);
let term4=Terms::new(z13.one(), index2.clone());
let m1= MultivariatePoly::new(vec![term0,term1,term2]);
let m2= MultivariatePoly::new(vec![term3,term4]);
let q=&m1/&m2;
let r = &m1%&m2;
let proof=&m2*&q+r;
assert_eq!(m1,proof);

}
}