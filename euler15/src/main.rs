extern crate num;

use num::FromPrimitive;
use num::bigint::BigUint;
use num::One;
use std::ops::Shl;

fn main() {
    let val: BigUint = BigUint::one();
    let digits = val.shl(1000).to_str_radix(10);
    let result:u32 = digits.chars().map(|c| c.to_digit(10).unwrap()).sum();
    println!("{}", result);
}
