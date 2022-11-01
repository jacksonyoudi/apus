use std::fmt::Debug;
use std::ops::Add;
use std::ops::Deref;
use std::convert::From;
use std::convert::Into;


pub trait Fly {
    fn fly(&self) -> bool;
}

#[derive(Debug)]
struct Duck;

#[derive(Debug)]
struct Pig;

impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

fn fly_static(s: impl Fly + Debug) -> bool {
    s.fly()
}

fn can_fly(s: impl Fly + Debug) -> impl Fly {
    if s.fly() {
        println!("{:?} can fly", s)
    } else {
        println!("{:?} can't fly", s)
    }
    s
}


// Implicit
// Explicit

fn main() {
    println!("Hello, world!");


    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let w = first_word(&s);
    println!("{}", w);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    let s = "Hello, world!";

    let s1: String = "test".to_string();

    let s2: &str = s1.as_str();

    let tup: (i32, f64, u8) = (500, 6.4, 1);
}


fn first_word(s: &String) -> &str {
    &s[..1]
}


fn sum<T>(a: impl Add<Output=T>, b: impl Add<Output=T>) -> T {
    a + b
}

#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new<T>(name: T) -> Person
        where T: Into<String>
    {
        Person {
            name: name.into(),
        }
    }
}
