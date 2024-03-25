#[macro_use]
pub mod poly;
pub mod polymod;

#[cfg(test)]
mod tests {
use num_bigint::BigInt;
use crate::intmod::PrimeField;
#[test]
fn test_add_poly() {
let z13=PrimeField(BigInt::from(13));
let p1 = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(2)),z13.new(BigInt::from(3)));
let p2 = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(2)));
println!("poly p1 is {}",p1);
println!("poly p2 is {}",p2);
println!("poly sum is {}",&p1+&p2);
println!("poly sub is {}",&p1-&p2);
println!("poly mul is {}",&p1*&p2);
println!("poly quotient is {}",&p1/&p2);
print!("poly reminder is {}",&p1%&p2);
let expected_add = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(7)),z13.new(BigInt::from(5)));
let expected_sub = poly!(z13.new(BigInt::from(1)), z13.new(BigInt::from(10)),z13.new(BigInt::from(1)));
let expected_mul = poly!(z13.new(BigInt::from(5)), z13.new(BigInt::from(12)),z13.new(BigInt::from(6)),z13.new(BigInt::from(6)));
let expected_quot=poly!(z13.new(BigInt::from(8)),z13.new(BigInt::from(5)));
let expected_rem=poly!(z13.new(BigInt::from(6)));

    assert_eq!(expected_add,&p1+&p2);
    assert_eq!(expected_sub,&p1-&p2);
    assert_eq!(expected_mul,&p1*&p2);
    assert_eq!(expected_quot,&p1/&p2);
    assert_eq!(expected_rem,&p1%&p2);

    }
}