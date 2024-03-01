
use std::{ops::{Add, Mul, Neg, Sub, Div}, fmt::{Debug,self}};
use num_bigint::Sign::Minus;
use num_bigint::RandBigInt;
use num_bigint::BigInt;
use num_traits::{Zero,One};
use crate::{integers::IntUtilities, field::Field};
#[derive(Debug,PartialEq)]
pub enum MathError {
    QuadraticNonResidueModP
}
pub type ModResult=Result<Mod,MathError>;
#[derive(Debug, Clone,Eq,PartialEq,PartialOrd, Ord)]
pub struct PrimeField(pub BigInt);
impl PrimeField {
    pub fn random(&self)->Mod{
        let mut rng = rand::thread_rng();
        let n= rng.gen_bigint_range(&BigInt::zero(),&self.0.clone());
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
        
        write!(f, "Mod({},{})", self.n,self.modulus.0.clone())
    }
}

/// # Example
/// ```
/// use algebra::intmod::PrimeField;
/// use algebra::field::Field;
/// use num_bigint::BigInt;
/// let z13=PrimeField(BigInt::from(13));
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
                let modulus=self.modulus.0;
                let mut sum = self.n+rhs.n;
                if sum>=modulus{
                    sum = sum-&modulus;
                    Mod::new(sum, PrimeField(modulus))}
                    else {Mod::new(sum, PrimeField(modulus))}}
                else {panic!("cannot add different modulus")}
}
}
impl<'a, 'b> Add<&'b Mod> for &'b Mod {
        type Output = Mod;
        fn add(self, rhs: Self) -> Self::Output {
            if self.modulus==rhs.modulus{
                let modulus=self.modulus.0.clone();
                let mut sum = &self.n+&rhs.n;
                if sum>=modulus{
                    sum = sum-&modulus;
                    Mod::new(sum, PrimeField(modulus))}
                    else {Mod::new(sum, PrimeField(modulus))}}
                else {panic!("cannot add different modulus")}
                
            }
    }
    impl<'a> Add<&'a Mod> for Mod {
        type Output = Mod;
        fn add(self, rhs: &'a Mod) -> Self::Output {
            if self.modulus==rhs.modulus{
                let modulus=self.modulus.0.clone();
                let mut sum = &self.n+&rhs.n;
                if sum>=modulus{
                    sum = sum-&modulus;
                    Mod::new(sum, PrimeField(modulus))}
                    else {Mod::new(sum, PrimeField(modulus))}}
                else {panic!("cannot add different modulus")}
                
            }
    }
impl Mul<Mod> for Mod {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            if self.modulus==rhs.modulus {
                Mod::new((self.n*rhs.n)%&self.modulus.0,rhs.modulus)}
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
impl <'a> Add<&'a mut Mod> for Mod {
    type Output=Mod;
    fn add(self, rhs: &'a mut Mod) -> Self::Output {
        if self.modulus==rhs.modulus{
            let modulus=self.modulus.0.clone();
            let mut sum = &self.n+&rhs.n;
            if sum>=modulus{
                sum = sum-&modulus;
                Mod::new(sum, PrimeField(modulus))}
                else {Mod::new(sum, PrimeField(modulus))}}
            else {panic!("cannot add different modulus")}
            
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

    impl<'a> Mul<Mod> for &'a mut Mod {
        type Output = Mod;
        fn mul(self, rhs: Mod) -> Self::Output {
            if rhs.modulus==self.modulus {
                Mod::new(&self.n*&rhs.n,self.modulus.clone())}
                else {panic!("You can not multiply two different mod!")}
        }
    }

impl Neg for Mod{
        type Output = Mod;
        fn neg(self) -> Self::Output {
            Mod::new(&self.modulus.0-&self.n,self.modulus.clone())
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
            let modulus=self.modulus.0.clone();
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
    fn is_one(&self)->bool{
        self.n.is_one()
    }
    fn inverse(&self)->Self {
            if self.n==BigInt::one(){return self.clone();}
            let a=self.n.clone();
            let b=self.modulus.0.clone();
            let bezout=BigInt::gcdext(a, b);
            if bezout[2]>BigInt::one() {panic!("Not an invertible element!")}
            else  {
                Mod::new(bezout[0].clone(), self.modulus.clone())
            }
        }
    
}
impl Mod {
    pub fn to_string(&self)->String{
        format!("Mod({},{})",self.n,self.modulus.0.clone())
    }
    pub fn new(mut n:BigInt,modulus:PrimeField)->Self{
        let prime=modulus.0.clone();
        while n.sign()==Minus {
            n=n+&prime;
        }
        Mod { n: n%&prime, modulus: modulus }
    }
    
    
/// This is Chinese Reminder Algorithm [10.52 H.Cohen Handbook of Elliptic and Hyperelliptic curves cryptography]
/// Given a Vec of Mod (x1, x2,...,xn) with coprime modulus, this algorithm computes z in the same 
/// residue classes as x1, x2,...,xn.
/// 
/// # Example
/// ```
/// use crate::algebra::integers::IntUtilities;
/// use crate::algebra::intmod::Mod;
/// use algebra::intmod::PrimeField;
/// use num_bigint::BigInt;
/// let f=Mod::new(BigInt::from(1),PrimeField(BigInt::from(3)));
/// let g=Mod::new(BigInt::from(2),PrimeField(BigInt::from(5)));
/// let h=Mod::new(BigInt::from(4),PrimeField(BigInt::from(7)));
/// let i=Mod::new(BigInt::from(5),PrimeField(BigInt::from(11)));
/// let j=Mod::new(BigInt::from(9),PrimeField(BigInt::from(13)));
/// let chinese=Mod::chinese(&[f,g,h,i,j].to_vec());
/// let expected_mod=Some(Mod { n: BigInt::from(8992), modulus:PrimeField(BigInt::from(15015))});
/// assert_eq!(expected_mod,chinese);
/// 
/// ```
    pub fn chinese(moduli:&Vec<Mod>)->Option<Mod>{
        let mut m=moduli[0].modulus.0.clone();
        let mut x=moduli[0].n.clone();
        
        for i in 1..moduli.len() {
            let [u,v,d]=BigInt::gcdext(moduli[i].modulus.0.clone(), m.clone());
            if d!=BigInt::one() {return None;} 
            x=&u*&moduli[i].modulus.0*x+&v*&m*&moduli[i].n;
            m=&m*&moduli[i].modulus.0;
            x=x%&m;               }
            Some(Mod::new(x, PrimeField(m)))
    }
    fn square(&mut self)->Self{
        let sqr=self.n.modpow(&BigInt::from(2),&self.modulus.0);
        Mod::new(sqr,self.modulus.clone())
    }
/// # Example
/// ```
/// use crate::algebra::integers::IntUtilities;
/// use crate::algebra::intmod::Mod;
/// use algebra::intmod::PrimeField;
/// use num_bigint::BigInt;
/// let field=PrimeField(BigInt::from(13i16));
/// let mut x=field.new(BigInt::from(7i16));
/// let exp = BigInt::from(-55i16);
/// let expected_power=field.new(BigInt::from(11));
/// assert_eq!(expected_power,x.pow_mod(&exp));
/// 
/// ```
    pub fn pow_mod(&mut self,exp:&BigInt)->Mod {
       /*Montgomery ladder */
       if exp.sign()==Minus {
        *self=self.inverse();
       }
       let bin=BigInt::to_binary(&exp);
       let mut p0=self.one();
       let mut p1=self.clone();
       for i in bin.iter() {
           if i.is_zero() {
               p1=&p1*&p0;
               p0=p0.square();
           }
           else {
               p0=p0*&p1;
               p1=p1.square();
               }
       }
       p0.clone()
    }
    pub fn check_sqrt_mod_prime(&mut self)->ModResult {
        if self.is_zero(){return Ok(self.zero());}
        let m=self.modulus.0.clone();
        let even_part=BigInt::even_part(&(&m-BigInt::one()));
        let q=(&m-BigInt::one())/even_part.0;
        let mut n=loop { 
            let rand = BigInt::random_8bit();
            if BigInt::kroneker(rand.clone(), m.clone())==-1i8 {break Mod::new(rand,PrimeField(m.clone()));}
        };
        let z=n.pow_mod(&q);
        let mut y = z;
        let mut r=even_part.1;
        let mut x= self.pow_mod(&((&q-BigInt::one())/BigInt::from(2)));
        
        let mut b=self.clone()*x.square();
        x=&x*&self;
        
        while !b.is_one(){
            let mut shift=1u64;
            let mut pow_two= BigInt::from(2);
        
            while  !b.pow_mod(&pow_two).is_one(){
                pow_two<<=1;
                shift+=1;   
            }

        
        if shift==r {return Err(MathError::QuadraticNonResidueModP);}
        
        let exp=r-shift-1;
        pow_two=BigInt::one()<<exp;
        let mut t=y.pow_mod(&pow_two);
        y=t.square();
        r=shift;
        x=&x*&t;
        b=&b*&y;
    }
    let neg_x=x.clone().neg();
    if x<neg_x {Ok(x)} else {Ok(-x)}
    }
/// # Example
/// ```
/// use crate::algebra::integers::IntUtilities;
/// use crate::algebra::intmod::Mod;
/// use algebra::intmod::PrimeField;
/// use num_bigint::BigInt;
/// let z17=PrimeField(BigInt::from(17));
/// let mut n=z17.new(BigInt::from(15));
/// let expected_sqrt=z17.new(BigInt::from(7));
/// assert_eq!(expected_sqrt,n.sqrt_mod_prime());
/// 
/// ```   
pub fn sqrt_mod_prime(&mut self)-> Mod{
    match self.check_sqrt_mod_prime(){
        Ok(sqrt)=> sqrt,
        Err(error)=>panic!("{:?}",error)
    }
}
}
