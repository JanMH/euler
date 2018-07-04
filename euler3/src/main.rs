fn is_prime(number: i64) -> bool {
    (2..sqrt_i(number)).all(|i| number % i != 0)
}

fn sqrt_i(i:i64) -> i64 {
    (i as f64).sqrt() as i64 + 1
}

fn main() {
    let to_test = 600851475143;
    let result = (2..sqrt_i(to_test)).filter(|i| to_test % i == 0).filter(|i| is_prime(*i));
    for i in result {
        println!("{}", i);
    }
}
