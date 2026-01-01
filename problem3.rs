fn main() {
    let num: u64 = 600851475143;
    let limit = (num as f64).sqrt() as usize;
    let mut prime = vec![true; limit + 1];

    let mut i = 2;

    while i * i <= limit {
        let mut j = i + i;
        while j <= limit {
            prime[j as usize] = false;
            j += i;
        }
        i += 1;
    }

    let mut j = prime.len() - 1;
    while j >= 2 {
        if prime[j] && num % j as u64 == 0 {
            println!("Largest prime: {j}");
            break;
        }
        j -= 1;
    }
}
