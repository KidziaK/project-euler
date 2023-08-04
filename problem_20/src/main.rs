use num_bigint::BigUint;

fn main() {
    let n = 100;
    let mut product = BigUint::parse_bytes(b"1", 10).unwrap();

    for i in 2..n + 1 {
        product *= BigUint::from(i as usize);
    }

    let sum_of_digits = product
        .to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();

    println!("{:?}", sum_of_digits);
}
