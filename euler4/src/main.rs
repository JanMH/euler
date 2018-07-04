fn disect(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut result = Vec::new();
    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }
    result
}

fn make_palindrome(n: i64) -> i64 {
    1000 * n + disect(n).iter().fold(0, |acc, x| 10 * acc + x)
}


fn sqrt_i(i: i64) -> i64 {
    (i as f64).sqrt() as i64 + 1
}


fn creatable(n: i64) -> bool {
    (sqrt_i(n)..999).rev().any(|i| n % i == 0)
}


fn main() {
    for i in (100..999).rev() {
        let pal = make_palindrome(i);
        if creatable(pal) {
            println!("{}", pal);
            return;
        }
    }
}
