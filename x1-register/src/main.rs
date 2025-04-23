struct User {
    username: String,
    password: String,
    is_admin: bool,
    age: u8
}

fn main() {
    let mut user1 = User {
        username: String::from("x0Greed"),
        password: String::from("SECRET"),
        is_admin: true,
        age: 0x19
    };

    println!("Username: {}\nPassword: {}\nIs Admin: {}\nAge: {}", user1.username, user1.password, user1.is_admin, user1.age);

    user1.username = String::from("x0Wrath");

    println!("<Username has been changed!>");
    println!("Username: {}", user1.username);
}