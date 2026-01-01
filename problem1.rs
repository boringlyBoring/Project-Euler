fn main() {
    let mut ans = 0;
    for i in 3..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            ans += i;
        }
    }
    println!("Answer: {ans}");
}
