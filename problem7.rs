fn main() {
    let n = 1_000_000;
    let mut prime: Vec<bool> = vec![true; n + 1];
    let mut p = 2;
    while p * p <= n {
        if prime[p] {
            let mut i = p * p;
            while i <= n {
                prime[i] = false;
                i += p;
            }
        }
        p += 1;
    }

    p = 2;
    let mut count = 0;
    while p <= n {
        if prime[p] {
            count += 1;
        }
        if count == 10_001 {
            println!("Answer: {p}");
            break;
        }
        p += 1;
    }
}
