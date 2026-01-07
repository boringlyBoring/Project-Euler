fn main() {
    let n = 100;

    let n_sum = (n * (n + 1)) / 2;
    let n_sq_sum = (n * (n + 1) * (2 * n + 1)) / 6;

    println!("{}", n_sum * n_sum - n_sq_sum);
}
