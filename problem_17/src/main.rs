use std::collections::HashMap;

fn main() {
    let n = 1001;

    let mut units = HashMap::from([
        (0, ""),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
    ]);

    let mut sum = 0;

    for i in 1..n {
        let thousands = i / 1000;
        let hundreds = (i % 1000) / 100;
        let tens = i % 100;

        sum += units.get(&thousands).unwrap().len();

        sum += units.get(&hundreds).unwrap().len();

        if units.contains_key(&tens) {
            sum += units.get(&tens).unwrap().len();
        } else {
            let unit = tens % 10;
            let ten = tens - unit;

            sum += units.get(&(ten)).unwrap().len() + units.get(&(unit)).unwrap().len();
        }

        if hundreds > 0 {
            if tens != 0 {
                // makeup for 'and'
                sum += 3;
            }

            // makeup for 'hundred'
            sum += 7;
        }

        if thousands > 0 {
            // makeup for 'thousand'
            sum += 8;
        }
    }

    println!("{}", sum);
}
