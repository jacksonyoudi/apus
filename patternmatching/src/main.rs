mod colors;
use crate::colors::ColoredString;
use crate::colors::Colorize;

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }


    let book: Book = Book {
        name: "hello",
        isbn: 13,
        version: 1,
    };
    let book1: Book = Book { version: 2, ..book };

    println!("{:?}", book);
    println!("{:?}", book1);

    let x: ColoredString = "hello".red().on_yellow();
    println!("{}", x)
}
// -- this field does not implement `Copy`
// &str 不可变借用
// value partially moved here

#[derive(Debug)]
struct Book<'a> {
    name: &'a str,
    isbn: i32,
    version: i32,
}