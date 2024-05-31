
use algebra::continued_functions::FiniteContinuedFunctions;
use termion::cursor::Goto;
use termion::{self, color, style};
use std::error::Error;
use std::io::{stdout, Read, Write};
use std::ops::Neg;
use std::{io, vec};
use algebra::multivariate::multiindex::MultiIndex;
use algebra::multivariate::terms::Terms;
use algebra::multivariate::multivariatepoly::{MultivariatePoly};
use algebra::poly;
use num_bigint::{BigUint,BigInt,RandomBits, ToBigInt};
use algebra::intmod::{Mod, PrimeField};
use num_traits::{int, One, Zero};
use num_rational::BigRational;


fn main() {
    let f=BigRational::new(BigInt::from(15),BigInt::from(11));
    let cf=FiniteContinuedFunctions::from_rational(f);
    println!("{}",cf);
/*     let z13=PrimeField(BigInt::from(13));

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
let mut m2= MultivariatePoly::new(vec![term4.clone(),term3.clone()]);//x_0*x_1-1

let m3= MultivariatePoly::new(vec![term2.clone(),term3.clone()]);//x_1^2-1
let multi_div= m1.multi_division_reminder(vec![m3.clone(),m2.clone()]);
println!("{m1} divided by {m2} and {m3}");
for item in &multi_div.0 {
    println!("quotient is {}",item);
}

println!("reminder is {}",multi_div.1);

let expected_q0=MultivariatePoly::new(vec![term5.clone(),term3.neg()]);
let expected_q1=MultivariatePoly::new(vec![term5]);
let expected_reminder = MultivariatePoly::new(vec![term7,term3.neg()]);
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



            /* /* PROMPT CMD */ 
            
            println!("{}{}",termion::clear::All,termion::cursor::Goto(1,1));       
            println!("{}{}{}Welcome to the Algebra!",color::Fg(color::Magenta),style::Italic,style::Bold);
            println!("{}Type 'quit' to exit.",color::Fg(color::LightYellow));   
            let mut line=1usize;
            loop {
                print!("{}{}in{line}: ",color::Fg(color::Blue),style::NoItalic);
                io::stdout().flush().unwrap();
                let mut input_string  =String::new();
                io::stdin().read_line(&mut input_string).expect("Failed to read the line");
                let commands=input_string.trim();
                if commands=="quit" { 
                    println!("{}Mandi biel!",color::Fg(color::LightMagenta));
                    break;
                }
                match evaluate_expression(&commands) {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => println!("Error: {}", err),
                }
                line+=1;


            }   
 */
    }

    fn evaluate_expression(expression:&str)->Result<BigInt,String> {
        
        let parts: Vec<&str> = expression.split_whitespace().collect();
        if parts.len() == 3 {
            let x=parts[0].parse::<BigInt>();
            let y=parts[2].parse::<BigInt>();
            let out:Result<BigInt, String> = match (x,y) {
                (Ok(num1),Ok(num2))=>{
                    let result = match parts[1] {
                        "+"=>num1+num2,
                        "-"=>num1-num2,
                        "*"=>num1*num2,
                        "/"=>num1/num2,
                        "%"=>num1%num2,
                        _ => return Err("Unsupported operator".to_string()),
                    };
                    Ok(result)
                }
                _=> return Err("Invalid operands".to_string()),
                
            };
            out
        } else {
            return Err("not binary operation".to_string())
        }
        
    }
    