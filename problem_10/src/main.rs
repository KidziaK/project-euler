fn main() {
    let n = 10;

    let mut is_prime: Vec<bool> = Vec::with_capacity(n);
    let mut prime_sum: u64 = 0;

    for _i in 0..n {
        is_prime.push(true);
    }

    for i in 2..n {
        if is_prime[i] {
            prime_sum += i as u64;

            for j in (2 * i..n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    println!("{}", prime_sum);
}
