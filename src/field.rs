use std::{ops::{Add, Div, Mul, Neg, Sub}, iter::Sum, fmt::Display};



pub trait Field
where
    Self:Sized
        +Clone
        +Display
        +PartialEq
        +PartialOrd
        +Eq
        + Add<Self, Output = Self>
        + Mul<Self, Output = Self>
        +for<'a> Mul<&'a Self,Output = Self>
        + Neg<Output = Self>
        + Sub<Self, Output = Self>
        + Div<Self, Output = Self>,
{
    fn one(&self)->Self;
    fn zero(&self)->Self;
    fn is_zero(&self)->bool;
    fn inverse(&self)->Self;
}


/* impl Field for Zmod {  
    fn zero(&self)->Self{
        let zero = Mod::new(BigInt::zero(), self.prime.clone());
        Zmod { num: Some(zero), prime: self.prime.clone() }
    }
    fn one(&self)->Self{
        let one = Mod::new(BigInt::one(), self.prime.clone());
        Zmod { num: Some(one), prime: self.prime.clone() }

    }
     
}
 */
