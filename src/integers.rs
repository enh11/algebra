use std::ops::BitAnd;
use num_bigint::BigInt;
use num_bigint::Sign::{Minus,Plus};
use num_traits::{Zero,One, Signed, ToPrimitive};
impl IntUtilities for BigInt {}

pub trait IntUtilities{

    fn to_binary(n:&BigInt)->Vec<u8>{
        format!("{:b}", n)
        .chars()
		.map(|x| if x == '1' { 1.into() } else { 0.into() })
		.collect()
    }
    fn is_even(a:&BigInt)->bool where Self: BitAnd<BigInt>,<Self as BitAnd<BigInt>>::Output: PartialEq<BigInt>{
        if (a & BigInt::one())== BigInt::zero(){true} else {false}
    }
    fn is_coprime(a:BigInt,b:BigInt)->bool{
        BigInt::gcd(a, b)==BigInt::one()
    }
/// This function gets the even part of a positive integer using shift.
/// It's out is the tuple (power_of_two,exponent).
/// 
/// # Example
/// ```
/// use crate::algebra::integers::IntUtilities;
/// use num_bigint::BigInt;
/// let a=BigInt::from(24u8);
/// let expected_value=(BigInt::from(8u8),3u64);
/// assert_eq!(BigInt::even_part(&a),expected_value);
/// ```
fn even_part(a:&BigInt)->(BigInt,u64) {
    let exp=a.trailing_zeros().unwrap();
    let v=(BigInt::one()<<exp).to_bytes_be();
    let pow=BigInt::from_signed_bytes_be(&v.1);
    (pow,exp)
}
fn compute_power_of_two(mut a:BigInt,mut b:BigInt)->(BigInt,BigInt,u64){
    let shift=(&a|&b).trailing_zeros().unwrap();
    a>>=&shift;
    b>>=&shift;
    (a,b,shift)
}
/// This is a binary algorithm that compute the greates common divisors.
/// The binary algorithm is especially well suited for computing GCD of multiprecision numbers.
/// This is because no division is performed: all division by 2 in this algorithm are performed using shift!
/// 
/// # Example
/// ```
/// use crate::algebra::integers::IntUtilities;
/// use num_bigint::BigInt;
/// let a=BigInt::from(60u8);
/// let b=BigInt::from(24u8);
/// let expected_value=BigInt::from(12u8);
/// assert_eq!(BigInt::gcd(a,b),expected_value);
/// ```
/// 
fn gcd(mut a:BigInt, mut b:BigInt)->BigInt{
    if a.sign()==Minus {a=-a}
    if b.sign()==Minus{b=-b}
    let k:u64;
    if a<b {
        std::mem::swap(&mut a,&mut b);
    }
    if b==BigInt::zero() {return a} else {
        let r=&a%&b;
        a=b;
        b=r;
    }
    if b==BigInt::zero(){return a} else {
        let z=Self::compute_power_of_two(a, b);
        a=z.0;
        b=z.1;
        k=z.2;
        }
    if BigInt::is_even(&a) {
        let shift=a.trailing_zeros().unwrap();
        a>>=shift;
    } else if BigInt::is_even(&b) {
            let shift=b.trailing_zeros().unwrap();
            b>>=shift;
        }
    loop{
    let mut t:BigInt=(&a-&b)>>1;
    if t==BigInt::zero() {
        let exp=(1u64<<k).to_be_bytes();
        return BigInt::from_signed_bytes_be(&exp)*a;
        }
        while BigInt::is_even(&t) {
            t=t/BigInt::from(2u8);
        };
        if t.sign()==Plus{
            a=t;
            } else {b=-t}

    }
}
/// # Example
/// ```
/// use crate::algebra::integers::IntUtilities;
/// use num_bigint::BigInt;
/// let a=BigInt::from(65u16);
/// let b=BigInt::from(26u16);
/// let expected_value=BigInt::from(130u16);
/// assert_eq!(BigInt::lcm(a,b),expected_value);
/// ```
fn lcm(a:BigInt,b:BigInt)->BigInt{
    &a*&b/BigInt::gcd(a, b)}
/// This is the euclidean extended binary algorithm that compute the greates common divisors.
/// given two integers a and b, the algorithms given below not only compute d = gcd(a, b)
///  but also the integers u and v such that au + bv = d.
/// 
/// # Example
/// ```
/// use crate::algebra::integers::IntUtilities;
/// use num_bigint::BigInt;
/// let a=BigInt::from(60u8);
/// let b=BigInt::from(24u8);
/// let z=BigInt::gcdext(a,b);
/// 
/// assert_eq!(z,[BigInt::from(-3),BigInt::from(8),BigInt::from(12)]);
/// ```
/// 
fn gcdext(mut a:BigInt,mut b:BigInt)->[BigInt;3]{
    let mut sign_a:BigInt=BigInt::one();
    let mut sign_b:BigInt=BigInt::one();
    if a.sign()==Minus {a=-a; sign_a=BigInt::from(-1)};
    if b.sign()==Minus{b=-b; sign_b=BigInt::from(-1)};

    let f1:bool;
    if a>b {
        std::mem::swap(&mut a, &mut b);
        f1=true;
    } else {f1=false;}

    let q=&b/&a;
    let r=&b%&a;
    let mut u=BigInt::zero();
    let mut v:BigInt;
    let d:BigInt;
    b=a;
    a=r;
    if a==BigInt::zero(){
        v=BigInt::one();
        d=b;
        if f1{return [u,v,d];} else {return [v,u,d];}
    }
    let mut k=0u64;
    let mut f= false;
    while BigInt::is_even(&b) && BigInt::is_even(&a) {
        let z =BigInt::compute_power_of_two(a, b);
        a=z.0;
        b=z.1;
        k=z.2;
    }
    if BigInt::is_even(&a){
        std::mem::swap(&mut a, &mut b);
        f=true;}
    
    let mut u_b=BigInt::one();
    let mut aux_a=b.clone();
    let mut aux_b=a.clone();
    let mut aux_v=a.clone();
    let mut u_a:BigInt;
    let mut aux_t:BigInt;
    
    if BigInt::is_even(&b)==false{
        u_a=BigInt::zero(); aux_t=-&a;
    } else {
        u_a=(BigInt::one()+&a)/2;
        aux_t=&b/BigInt::from(2);}
    
    while aux_t !=BigInt::zero() {

        if aux_t>BigInt::zero() {
            u_b=u_a;aux_a=aux_t;} else {
            aux_b=&a-&u_a;aux_v=-&aux_t;
        }

        u_a=&u_b-&aux_b;
        aux_t=&aux_a-&aux_v;

        if u_a<BigInt::zero(){
            u_a=&u_a+&a;}
        while BigInt::is_even(&aux_t) && aux_t != BigInt::zero() {
            aux_t=&aux_t/2;
            if BigInt::is_even(&u_a) {u_a=&u_a/2;} else {u_a=(&u_a+&a)/BigInt::from(2);}      
        }

        }
    u=u_b.clone();
    let v_exp=(BigInt::one()<<k).to_bytes_be();
    d=BigInt::from_signed_bytes_be(&v_exp.1)*&aux_a;   
    v=(&aux_a-&b*&u_b)/&a; 
    if f {std::mem::swap(&mut u, &mut v);}
    u=u-&v*&q; 
    if f1 {return [sign_b*v,sign_a*u,d];} else {return [sign_a*u,sign_b*v,d];}  
}
fn check_gcdext( a:BigInt,b:BigInt){
let v=BigInt::gcdext(a.clone(),b.clone());
assert_eq!(a*&v[0]+b*&v[1],v[2])
}
const TAB2:[i8;8]=[0,1,0,-1,0,-1,0,1];
fn kroneker<'a>(mut a:BigInt,mut b:BigInt)->i8 {
    let mut k:i8;
    if b.is_zero(){
        if !a.abs().is_one() {return 0i8;} else { return 1i8;}
    }
    if BigInt::is_even(&a) && BigInt::is_even(&b) {return 0i8;}
    let mut v= 0;
    let mut even_part:(BigInt,u64);
    if BigInt::is_even(&b){
        even_part=BigInt::even_part(&b);

        b=b>>even_part.1;
        v=even_part.1;
        }
    if v%2==0 {k = 1;} else {
        k = Self::TAB2[(&a&BigInt::from(7)).to_usize().unwrap()];
        }
    if b.sign()==Minus {
        b=-b;
        if a.sign()==Minus {k=-k;}
    }
    loop{
    if a==BigInt::zero(){ 
        if b>BigInt::one() {break 0;} 
        else {break k;}}
        v=0;
        if BigInt::is_even(&a){
    even_part=BigInt::even_part(&a);
    a=a>>even_part.1;
    v=even_part.1;
    }
    if v%2==1 {
        k = Self::TAB2[(&b&BigInt::from(7)).to_usize().unwrap()]*k;

    }
    if (&a&(&b)&BigInt::from(2))!=BigInt::zero() {
        k=-k;
        }
        let r = a.abs();
        a=b%&r;
        b=r;

    }
}
}
