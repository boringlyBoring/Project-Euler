use std::{cmp, time::Instant};

//Largest palindrome by multiplication
//of two 3 digit numbers
fn solve() -> i64 {
    let mut max_num = -1;
    for i in 111..=999 {
        for j in i..=999 {
            let num = i * j;
            if palindrome((i * j).to_string()) {
                max_num = cmp::max(max_num, num);
            }
        }
    }
    max_num
}

fn palindrome(num: String) -> bool {
    let mut l: usize = 0;
    let mut r: usize = num.len() - 1;
    let chars: Vec<char> = num.chars().collect();
    while l <= r {
        if chars[l] != chars[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

fn main() {
    let start = Instant::now();
    println!("Answer: {}", solve());
    println!("Time taken: {:?}", start.elapsed());
}
