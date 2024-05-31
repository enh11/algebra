use core::fmt;
use std::fmt::Display;
use itertools::Itertools;
use nalgebra::{Matrix, Matrix2, RowVector2};

use num_bigint::BigInt;
use num_rational::{BigRational, Rational};
use num_traits::{One, Zero};
use rustc_serialize::json::Array;
#[derive(Debug,Clone,Eq,PartialEq,PartialOrd, Ord)]

pub struct FiniteContinuedFunctions{
    pub partial_quotients:Vec<BigInt>
}
impl FiniteContinuedFunctions{
   pub fn new(q:Vec<BigInt>)->Self{
        FiniteContinuedFunctions{
            partial_quotients:q
        }
    }
    pub fn from_rational(rational:BigRational)->Self{
        
        let mut partial_quotients:Vec<BigInt>=Vec::new();
        let  [mut q,mut r]=quotient_reminder(&rational);
        partial_quotients.push(q);
        let mut num=rational.denom();
        let mut den=&r;
        let mut new_rational:BigRational;

        if !r.is_zero(){
        new_rational=BigRational::new(num.clone(), den.clone());}else {
            return     FiniteContinuedFunctions::new(partial_quotients );
        }

        loop{
            
            [q,r]=quotient_reminder(&new_rational);
            partial_quotients.push(q);
            if r.is_zero(){break;}
            num=new_rational.denom();
            new_rational=BigRational::new(num.clone(),r.clone());
            
        }

    FiniteContinuedFunctions::new(partial_quotients ) 
}
pub fn to_rational(&self)->BigRational{
    let pq_matrix :Matrix2<BigInt>= self.partial_quotients.iter().map(|a|
        Matrix2::from_rows(&[RowVector2::new(a.clone(), BigInt::one()),
        RowVector2::new(BigInt::one(),BigInt::zero())])).product();
    let first_column =pq_matrix.column(0);
    BigRational::new(first_column.x.clone(), first_column.y.clone())
    
    
        
}
}
impl Display for FiniteContinuedFunctions{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
               
        write!(f,"{:?}",self.partial_quotients)}
   
}
pub fn quotient_reminder(r:&BigRational)->[BigInt;2]{
    let quotient=r.numer()/r.denom();
    let rem = r.numer()%r.denom();
    [quotient,rem]
}