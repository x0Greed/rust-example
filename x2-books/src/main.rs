fn main() {

    /*----------------*/

    let mut positive_integers = vec![1,2,3,4,5];

    println!("Length is {}", positive_integers.len());

    positive_integers.push(6);

    println!("<Length has been changed!>");
    println!("Length is {}", positive_integers.len());
    println!(".....");

    /*----------------*/

    let mut books = Vec::<String>::new();

    books.push(String::from("The Lord of the Rings"));
    books.push(String::from("Rubaiyat of Omar Khayyam"));

    for (i, book) in books.iter().enumerate() {
        println!("{}: {}", i + 1, book);
    }

    /*----------------*/
}