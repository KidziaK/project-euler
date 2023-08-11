use itertools::Itertools;

fn main() {
    let n = 1000000;

    let items = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut permutations = items.iter().permutations(items.len());

    println!(
        "{:?}",
        permutations
            .nth(n - 1)
            .unwrap()
            .iter()
            .map(|i| i.to_string())
            .join("")
            .parse::<u64>()
            .unwrap()
    );
}
