use itertools::iproduct;

fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::<usize>::with_capacity(n / 2);

    for i in 1..n / 2 + 1 {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    return divisors;
}

const CAP: usize = 28123;

fn main() {
    let mut abundant = Vec::<usize>::new();

    for n in 1..CAP {
        if divisors(n).iter().sum::<usize>() > n {
            abundant.push(n);
        }
    }

    let mut expressible = [false; CAP + 1];

    let cartesian: Vec<(usize, usize)> =
        iproduct!(abundant.to_vec(), abundant).collect::<Vec<(usize, usize)>>();

    for (x, y) in cartesian {
        if x + y <= CAP {
            expressible[x + y] = true;
        }
    }

    println!(
        "{:?}",
        expressible
            .iter()
            .enumerate()
            .map(|(i, &x)| i * (1 - x as usize))
            .collect::<Vec::<usize>>()
            .iter()
            .sum::<usize>()
    );
}
