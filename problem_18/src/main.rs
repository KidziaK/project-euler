use std::fs;

fn main() {
    let file_path = "src/triangle.txt";

    let contents = fs::read_to_string(file_path).expect("File does not exists");

    let mut triangle = contents
        .split("\n")
        .map(|row| {
            row.split(" ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    for (i, level) in triangle.clone().iter().enumerate().rev() {
        for j in 0..level.len() - 1 {
            triangle[i - 1][j] = triangle[i - 1][j] + triangle[i][j].max(triangle[i][j + 1]);
        }
    }

    println!("{}", triangle[0][0]);
}
