use std::fs;

fn main() {
    let digits = 10;
    let file_path = "src/numbers.txt";
    let file_contents = fs::read_to_string(file_path).expect("No such file");
    let numbers = file_contents.split("\n");

    let mut result = Vec::<usize>::new();
    let mut carry: usize = 0;

    for digit in (0..50).rev() {
        let mut sum = carry;

        for num in numbers.clone() {
            sum += num.chars().nth(digit).unwrap().to_digit(10).unwrap() as usize;
        }

        carry = sum / 10;
        result.push(sum % 10);
    }

    for d in carry.to_string().chars().rev() {
        result.push(d.to_digit(10).unwrap() as usize);
    }

    result.reverse();

    println!(
        "{:?}",
        result
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")[0..digits]
            .to_string()
    );
}
