use std::collections::HashMap;
use std::num::ParseIntError;

fn main() {
    println!("Hello, world!");
    let mut book_reviews: HashMap<&str, &str> = HashMap::with_capacity(10);
    book_reviews.insert("Rust Book", "good");
    book_reviews.insert("programing Rust", "nice");
    book_reviews.insert("The Tao of Rust", "deep");

    for key in book_reviews.keys() {
        println!("key {}", key)
    }

    if !book_reviews.contains_key("rust book") {
        println!("find {} times ", book_reviews.len())
    }

    book_reviews.remove("Rust Book");
    let to_find: [&str; 2] = ["Rust Book", "The Tao of Rust"];
    for x in to_find.into_iter() {
        match book_reviews.get(x) {
            Some(review) => println!("{}: {}", x, review),
            None => println!("{} is unreviewed .", x)
        }
    }

    for (book, review) in &book_reviews {
        println!("{}: \"{}\"\n", book, review)
    }

    assert_eq!(book_reviews["The Tao of Rust"], "deep")

}


fn square(numer_str: &str) -> Result<i32, ParseIntError> {
    numer_str.parse::<i32>().map(|n| n.pow(2))
}


