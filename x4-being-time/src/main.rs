use std::collections::HashSet;

fn main() {
    let mut world: HashSet<String> = HashSet::new();

    world.insert(String::from("being"));
    world.insert(String::from("time"));

    if world.contains("being") == true && world.contains("time") == true {
        println!("the world was created");
    } else if world.contains("being") != true || world.contains("time") != true {
        println!("error");
    } else {
        println!("the world was not created");
    }
}