use std::time::Instant;

fn solve() -> u64 {
    let mut num: u64 = 600851475143;
    if num % 2 == 0 {
        num = num / 2;
    }

    if num == 1 {
        return 2;
    }

    let mut d: u64 = 3;
    while d * d <= num {
        while num % d == 0 {
            num = num / d;
        }
        d += 2;
    }
    num
}

fn main() {
    let start = Instant::now();
    println!("{}", solve());
    let time_elapsed = start.elapsed();
    println!("Time taken: {:?}", time_elapsed);
}
