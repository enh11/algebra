
use core::fmt;
use std::{ops::{Add, Mul, Neg, Sub, Div, SubAssign}, fmt::Debug, process::Output, clone};
use num_bigint::Sign::Minus;
use num_bigint::RandBigInt;
use num_bigint::BigInt;
use num_traits::{Zero,One};
use crate::{integers::IntUtilities, field::Field};

#[derive(Debug, Clone,Eq,PartialEq,PartialOrd, Ord)]
pub struct PrimeField(pub Option<BigInt>);
impl PrimeField {
    pub fn random(&self)->Mod{
        let mut rng = rand::thread_rng();
        let n= rng.gen_bigint_range(&BigInt::zero(),&self.0.clone().unwrap());
        Mod::new(n, self.clone())
    }
    pub fn new(&self,num:BigInt)->Mod{
        Mod::new(num,self.clone())
    }
    pub fn zero(&self)-> Mod{
        Mod::new(BigInt::zero(),self.clone())
    }
    pub fn one(&self)->Mod{
        Mod::new(BigInt::one(),self.clone())
    }
}
#[derive(Debug, Clone,Eq,PartialEq,PartialOrd, Ord)]
pub struct Mod {
    pub n:BigInt,
    pub modulus:PrimeField
}
impl fmt::Display for Mod {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "Mod({},{})", self.n,self.modulus.0.clone().unwrap())
    }
}
/// # Example
/// ```
/// use algebra::intmod::PrimeField;
/// use algebra::field::Field;
/// use num_bigint::BigInt;
/// let z13=PrimeField(Some(BigInt::from(13)));
/// let x=z13.new(BigInt::from(12));
/// let y=z13.new(BigInt::from(3));
/// let expected_add=z13.new(BigInt::from(2));
/// let expected_sub=z13.new(BigInt::from(9));
/// let expected_mul=z13.new(BigInt::from(10));
/// let expected_div=z13.new(BigInt::from(4));
/// 
/// let add=&x+&y;
/// let sub=&x-&y;
/// let mul=&x*&y;
/// let div=&x/&y;
/// 
/// assert_eq!(expected_add,add);
/// assert_eq!(expected_sub,sub);
/// assert_eq!(expected_mul,mul);
/// assert_eq!(expected_div,div);   
/// ```
impl Add<Mod> for Mod {
    type Output = Mod;
    
    fn add(self, rhs: Self) -> Self::Output {
        if self.modulus==rhs.modulus{
                let modulus=self.modulus.0.unwrap();
                let mut sum = self.n+rhs.n;
                if sum>=modulus{
                    sum = sum-&modulus;
                    Mod::new(sum, PrimeField(Some(modulus)))}
                    else {Mod::new(sum, PrimeField(Some(modulus)))}}
                else {panic!("cannot add different modulus")}
}
}
impl<'a, 'b> Add<&'b Mod> for &'b Mod {
        type Output = Mod;
        fn add(self, rhs: Self) -> Self::Output {
            if self.modulus==rhs.modulus{
                let modulus=self.modulus.0.clone().unwrap();
                let mut sum = &self.n+&rhs.n;
                if sum>=modulus{
                    sum = sum-&modulus;
                    Mod::new(sum, PrimeField(Some(modulus)))}
                    else {Mod::new(sum, PrimeField(Some(modulus)))}}
                else {panic!("cannot add different modulus")}
                
            }
    }
impl Mul<Mod> for Mod {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            if self.modulus==rhs.modulus {
                Mod::new((self.n*rhs.n)%&self.modulus.0.unwrap(),rhs.modulus)}
                else {panic!("You can not multiply to different mod!")}
        }
    }
impl<'a,'b> Mul<&'b Mod> for &'b Mod {
        type Output = Mod;
        fn mul(self, rhs: Self) -> Self::Output {
            if rhs.modulus==self.modulus {
                Mod::new(&self.n*&rhs.n,self.modulus.clone())}
                else {panic!("You can not multiply two different mod!")}
        }
    }

    impl<'a> Mul<&'a Mod> for Mod {
        type Output = Mod;
        fn mul(self, rhs: &'a Mod) -> Self::Output {
            if rhs.modulus==self.modulus {
                Mod::new(&self.n*&rhs.n,self.modulus.clone())}
                else {panic!("You can not multiply two different mod!")}
        }
    }

impl Neg for Mod{
        type Output = Mod;
        fn neg(self) -> Self::Output {
            match self.modulus.0.clone() {
                Some(prime)=>Mod::new(prime-&self.n,self.modulus.clone()),
                None=>panic!("There's no inverse!")
            }
        }
    }
impl Sub<Mod> for Mod {
        type Output = Mod;
        fn sub(self, rhs: Self) -> Self::Output {
            if rhs.modulus==self.modulus {
            self+rhs.neg()} 
            else {panic!("You can not subtruct two differents mod!")}
    
        }   
    }
impl<'a, 'b> Sub<&'b Mod> for &'b Mod {
    type Output = Mod;
        fn sub(self, rhs: Self)->Mod {
            let mut aux=self.n.clone();
            let modulus=self.modulus.0.clone().unwrap();
                if rhs.modulus==self.modulus {
                    if self.n<rhs.n {
                         aux=&self.n+modulus;
                    }
                self.modulus.new(aux-&rhs.n)
                }
                else {panic!("You can not subtract two different mod!")}
                
            }
    }
impl Div<Mod> for Mod {
        type Output = Mod;
        fn div(self:Self,other:Self)->Self::Output{
            self*other.inverse()
        }
    }
impl<'a, 'b> Div<&'b Mod> for &'b Mod{
        type Output = Mod;
        fn div(self:Self,other:Self)->Self::Output{
            self*&other.inverse()
        }
    }
    
impl Field for Mod {
    fn one(&self)->Self {
        Mod::new(BigInt::one(), self.modulus.clone())
    }
    fn zero(&self)->Self {
        Mod::new(BigInt::zero(), self.modulus.clone())

    }  
    fn is_zero(&self)->bool{
        self.n.is_zero()
    }
    fn inverse(&self)->Self {
            let a=self.n.clone();
            let b=self.modulus.0.clone().unwrap();
            let bezout=BigInt::gcdext(a, b);
            if bezout[2]>BigInt::one() {panic!("Not an invertible element!")}
            else  {
                Mod::new(bezout[0].clone(), self.modulus.clone())
            }
        }
    
}
impl Mod {
    pub fn to_string(&self)->String{
        format!("mod({},{})",self.n,self.modulus.0.clone().unwrap())
    }
    pub fn new(mut n:BigInt,modulus:PrimeField)->Self{
        let prime=modulus.0.clone().unwrap();
        while n.sign()==Minus {
            n=n+&prime;
        }
        Mod { n: n%&prime, modulus: modulus }
    }
    
    
/// This is Chinese Reminder Algorithm [10.52 H.Cohen Handbook of Elliptic and Hyperelliptic curves cryptography]
/// Given a Vec of Mod with coprime modulus, this algorithm finds a class Mod(x,m) such that x=x_i mod m_i for each Mod(x_i,m_i)
/// in the input vector.
/// 
/// # Example
/// ```
/// use crate::algebra::integers::IntUtilities;
/// use crate::algebra::intmod::Mod;
/// use algebra::intmod::PrimeField;
/// use num_bigint::BigInt;
/// let f=Mod::new(BigInt::from(1),PrimeField(Some(BigInt::from(3))));
/// let g=Mod::new(BigInt::from(2),PrimeField(Some(BigInt::from(5))));
/// let h=Mod::new(BigInt::from(4),PrimeField(Some(BigInt::from(7))));
/// let i=Mod::new(BigInt::from(5),PrimeField(Some(BigInt::from(11))));
/// let j=Mod::new(BigInt::from(9),PrimeField(Some(BigInt::from(13))));
/// let chinese=Mod::chinese(&[f,g,h,i,j].to_vec());
/// let expected_mod=Some(Mod { n: BigInt::from(8992), modulus:PrimeField(Some(BigInt::from(15015)))});
/// assert_eq!(expected_mod,chinese);
/// 
/// ```
/// 




/// 
/// assert_eq!(z,[BigInt::from(-3),BigInt::from(8),BigInt::from(12)]);
/// ```
    pub fn chinese(moduli:&Vec<Mod>)->Option<Mod>{
        let mut m=moduli[0].modulus.0.clone().unwrap();
        let mut x=moduli[0].n.clone();
        
        for i in 1..moduli.len() {
            let [u,v,d]=BigInt::gcdext(moduli[i].modulus.0.clone().unwrap(), m.clone());
            if d!=BigInt::one() {return None;} 
            x=&u*moduli[i].modulus.0.as_ref().unwrap()*x+&v*&m*&moduli[i].n;
            m=&m*moduli[i].modulus.0.as_ref().unwrap();
            x=x%&m;               }
            Some(Mod::new(x, PrimeField(Some(m))))
    }
}
