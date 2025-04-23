use std::collections::HashMap;

fn main() {
    let mut characters = HashMap::from([
        ("Genghis Khan", "1162"),
        ("Börte", "1161"),
    ]);

    characters.insert("Yesugei", "1134");

    for (name, date_of_birth) in characters {
        println!("{}, {}", name, date_of_birth);
    }
}