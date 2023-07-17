fn main() {
    let n = 10001;

    let mut is_prime: Vec<bool> = Vec::with_capacity(n*n);
    let mut primes: Vec<usize> = Vec::with_capacity(n);

    for _i in 0..n*n {
        is_prime.push(true);
    }

    for i in 2..n*n {
        if is_prime[i] {
            primes.push(i);

            for j in (2*i..n*n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    println!("{}", primes[10000]);
}
