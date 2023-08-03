use time::{Date, Month, Weekday};

fn main() {
    let mut sundays = 0;

    let mut starting_date =
        Date::from_calendar_date(1901, Month::January, 1).expect("Incorrect starting_date");
    let ending_date =
        Date::from_calendar_date(2000, Month::December, 31).expect("Incorrect ending_date");

    while starting_date <= ending_date {
        starting_date = starting_date.next_occurrence(Weekday::Sunday);

        if starting_date.day() == 1 {
            sundays += 1;
        }
    }

    if starting_date.day() == 1 {
        sundays += 1;
    }

    println!("{sundays}")
}
