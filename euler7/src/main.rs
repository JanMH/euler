
fn is_prime(number: i64) -> bool {
    number == 2 || (2..sqrt_i(number)).all(|i| number % i != 0)
}

fn sqrt_i(i:i64) -> i64 {
    (i as f64).sqrt() as i64 + 1
}

fn main() {
    let mut result = (2..).filter(|x| is_prime(*x));

    
    let mut result = result.skip(10000);
   
    println!("{}", result.next().unwrap());
}
