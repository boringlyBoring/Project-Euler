fn main() {
    let mut sum: u64 = 0;
    let mut num = 1;
    loop {
        if num > 872_000 {
            break;
        }
        let sq_num = num * num;
        if sq_num % 2 == 1 {
            sum += sq_num;
        }
        num += 1;
    }
    println!("Answer: {sum}");
}
