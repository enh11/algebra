use std::{ops::{Add, Div, Mul, Neg, Sub},fmt::{Display, Debug}};
pub trait Field
where
    Self:Sized
        +Debug
        +Clone
        +Display
        +PartialEq
        +Ord
        +PartialOrd
        +Eq
        + Add<Self, Output = Self>
        +for<'a> Add<&'a Self,Output = Self>
        +for<'a> Add<&'a mut Self, Output = Self>
        +for<'a> Add<Self,Output = Self>
        +for<'a> Add<&'a Self>
        + Mul<Self, Output = Self>
        +for<'a> Mul<&'a Self,Output = Self>
        +for<'a> Mul<Self,Output = Self>
        + Neg<Output = Self>
        + Sub<Self, Output = Self>
        + Div<Self, Output = Self>,
{
    fn one(&self)->Self;
    fn zero(&self)->Self;
    fn is_zero(&self)->bool;
    fn is_one(&self)->bool;
    fn inverse(&self)->Self;
}
