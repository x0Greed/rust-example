fn main() {
    add(2, 8);

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn add(a: i32, b: i32) {
    println!("{a} + {b}: {}", a + b);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
