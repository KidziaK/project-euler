use std::fs;

const N: usize = 20;

fn main() {
    let file_path = "src/grid.txt";
    let file_contents = fs::read_to_string(file_path).expect("No such file");

    let mut grid = [[0; N]; N];

    for (i, row) in file_contents.split("\n").enumerate() {
        for (j, column) in row.split(" ").enumerate() {
            grid[i][j] = column
                .parse::<i32>()
                .expect("There was a string that is not a number!");
        }
    }

    let k = 4;
    let mut max_product = 1;

    for i in 0..20 - k {
        for j in 0..20 - k {
            let mut prod = 1;

            for l in 0..k {
                prod *= grid[i + l][j];

                if prod > max_product {
                    max_product = prod;
                }
            }

            prod = 1;

            for l in 0..k {
                prod *= grid[i][j + l];

                if prod > max_product {
                    max_product = prod;
                }
            }

            prod = 1;

            for l in 0..k {
                prod *= grid[i + k - l][j + l];

                if prod > max_product {
                    max_product = prod;
                }
            }
        }
    }

    println!("{}", max_product);
}
