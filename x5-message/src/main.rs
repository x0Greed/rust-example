enum Message {
    Value { id: i32 },
}

fn main() {
    let msg = Message::Value { id: 17 };

    match msg {
        Message::Value { id: id_variable @ 3..=7 } => {
            println!("id: {}", id_variable);
        }

        Message::Value { id: 10..=20 } => {
            println!("id is in the range 10 to 20");
        }

        _ => {
            println!("id is out of range");
        }
    }
}
