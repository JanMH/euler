
fn iteration(c:i32) -> bool {
    
    for x in 1..  {
        let b = 1000 - c - x;
        let a = 1000 - b -c;
        if a*a + b*b == c*c {
            println!("{}, {}, {}, a*b*c: {}",a,b,c, a*b*c);
            return true;
        }
        if  a >= b  {break;}
    }
    false
}

fn main() {
    let mut i = 335i32;
    while ! iteration(i) { i += 1;}
}
