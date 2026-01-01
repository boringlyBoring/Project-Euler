fn main() {
    let mut a: u64 = 1;
    let mut b: u64 = 2;
    let mut sum: u64 = 2;
    loop {
        let c = a + b;
        if c > 4_000_000 {
            break;
        }
        if c % 2 == 0 {
            sum += c;
        }
        a = b;
        b = c;
    }
    println!("Answer: {sum}");
}
