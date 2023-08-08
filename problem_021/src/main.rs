fn divisors(n: usize) -> Vec<usize> {
    let sqrt_n = (n as f64).sqrt() as usize;
    let mut divisors = Vec::<usize>::with_capacity(sqrt_n);

    for i in 1..n / 2 + 1 {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    return divisors;
}

fn main() {
    let max_n = 10000;
    let mut amicable = Vec::<usize>::new();

    for n in 1..max_n + 1 {
        let c = divisors(n).iter().sum::<usize>();
        let cc = divisors(c).iter().sum::<usize>();

        if (n == cc) & (n != c) {
            amicable.push(n);
        }
    }

    println!("{}", amicable.iter().sum::<usize>());
}
