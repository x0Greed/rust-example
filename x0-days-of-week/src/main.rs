const DAYS_OF_WEEK: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday"
];

fn main() {
    for day in DAYS_OF_WEEK.iter() {
        println!("{}", day);
    }
}