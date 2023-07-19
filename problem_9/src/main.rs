fn main() {

    // TODO reduce the number of checks, this brute force is dumb
    for a in 1..1000 {
        for b in a..1000 {
            let c = 1000 - a - b;

            if a*a + b*b == c*c {
                println!("abc = {}", a * b * c);
            }
        }
    }
}
