use num_bigint::BigUint;

const DIGITS: usize = 1000;

fn main() {
    let mut idx = 2;

    let mut fib1 = BigUint::parse_bytes(b"1", 10).unwrap();
    let mut fib2 = BigUint::parse_bytes(b"1", 10).unwrap();

    while fib2.to_string().len() < DIGITS {
        let temp = fib2.clone();
        fib2 += fib1;
        fib1 = temp;

        idx += 1;
    }

    println!("F({}) = {}", idx, fib2);
}
