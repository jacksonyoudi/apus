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