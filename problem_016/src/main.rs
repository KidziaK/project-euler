use num_bigint::BigUint;

fn main() {
    let n: BigUint = BigUint::parse_bytes(b"2", 10).unwrap().pow(1000);

    println!(
        "{}",
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>()
    );
}
