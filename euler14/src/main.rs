fn collatz_len(n: u64) -> usize {
    let mut current_collatz_number = n;

    let mut len = 0;

    while current_collatz_number != 1 {
        current_collatz_number = if current_collatz_number % 2 == 0 {
            current_collatz_number / 2
        } else {
            3 * current_collatz_number + 1
        };
        len += 1;
    }
    len
}

fn main() {
    let iter = (1..1000000).map( |a| (collatz_len(a),a));
    let result = iter.max_by(|&(a,_),&(b,_)| a.cmp(&b)).unwrap();
    println!("{:?}", result);
}
