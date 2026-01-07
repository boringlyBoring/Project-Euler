fn solve(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return solve(b, a % b);
}

fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 0;

    while b <= 20 {
        c = (a * b) / solve(a, b);
        a = c;
        b += 1;
    }
    println!("{c}");
}
