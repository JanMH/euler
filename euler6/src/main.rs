fn main() {

    let fst = (0..101i64).fold(0, |acc, x| acc + x);
    let fst = fst * fst;
    let snd = (0..101i64).fold(0, |acc, x| acc + x * x);

    println!("{}", fst - snd);
}
