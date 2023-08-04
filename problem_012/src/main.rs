const MAX_ITER: usize = 100000;

fn find_factors(n: usize) -> Vec<usize> {
    let sqrt_n = (n as f64).sqrt() as usize;
    let mut factors = Vec::<usize>::with_capacity(sqrt_n);

    for i in 1..sqrt_n + 1 {
        if n % i == 0 {
            factors.push(i);

            if i != n / i {
                factors.push(n / i);
            }
        }
    }

    return factors;
}

fn nth_triangular(n: usize) -> usize {
    return n * (n + 1) / 2;
}

fn main() {
    let required_divisors = 500;
    let mut i = 1;

    while i < MAX_ITER {
        let triangular_number = nth_triangular(i);
        let factors = find_factors(triangular_number);

        if factors.len() >= required_divisors {
            println!("Number: {}", triangular_number);
            break;
        }

        i += 1;
    }
}
