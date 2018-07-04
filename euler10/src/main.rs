fn main() {
    let mut buffer: Vec<i64> = Vec::with_capacity(1000000);
    buffer.push(2);
    let mut sum:i64 = 2;
    for number in 2..2000000i64 {
        if is_prime_buffered(number,&buffer) {
            buffer.push(number);
            sum += number;
        }
    }
    println!("{}", sum);
}


fn is_prime_buffered(x:i64, primes: &Vec<i64>) -> bool {
    let sqrt_x = (x as f64).sqrt();
    let sqrt_x = sqrt_x as i64 + 1;
    for y in primes {
        if(*y > sqrt_x) { return true; }
        if(x % *y == 0) { return false; }

    }
    return true; 
}

fn is_prime(x:&i64) -> bool {
    let sqrt_x = ((*x) as f64).sqrt();
    let sqrt_x = sqrt_x as i64 + 1;
    (2..sqrt_x).all(|y| x % y != 0)
}