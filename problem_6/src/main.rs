fn square_sum_minus_sum_squared(n: i32) -> i32 {
    return n.pow(4) / 4 + n.pow(3) / 6 - n.pow(2) / 4 - n / 6;
}

fn main() {
    let n = 100;

    println!("{}", square_sum_minus_sum_squared(n));
}
