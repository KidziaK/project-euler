use std::fs;
use std::cmp;

fn main() {
    let file_path = "src/big_number.txt";
    let mut number_str: String = String::new();


    match fs::read_to_string(file_path) {
        Ok(contents) => number_str = contents.replace("\n", ""),
        Err(_) => eprintln!("No such file: {}", file_path),
    }

    let adjecent = 13;
    let mut max_product: u64 = 1;

  
    for n in 0..number_str.len()-adjecent+1 {
        let mut product: u64 = 1;

        for i in n..n+adjecent {
            product *= number_str.chars().nth(i).unwrap().to_digit(10).unwrap() as u64;
        }

        max_product = cmp::max(max_product, product);
    }

    println!("{}", max_product);
}
