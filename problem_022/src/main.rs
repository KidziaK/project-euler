use std::fs;

fn name_score(name: String) -> u32 {
    let mut sum = 0;

    for c in name.chars() {
        sum += c as u32 - 'A' as u32 + 1;
    }

    return sum;
}

fn main() {
    let file_path = "src/names.txt";

    let contents = fs::read_to_string(file_path).expect("File does not exists!");

    let mut names = contents
        .split(",")
        .map(|s| s.to_string().replace("\"", ""))
        .collect::<Vec<String>>();

    names.sort();

    let mut sum = 0;

    for (i, name) in names.iter().enumerate() {
        sum += (i as u32 + 1) * name_score(name.to_string());
    }

    println!("{}", sum);
}
