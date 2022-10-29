use std::cmp::Ordering;
use std::io;
// prelude
use rand::Rng; // trait

fn main() {
    println!("猜数!");
    println!("猜测一个数");

    let i: i32 = rand::thread_rng().gen_range(1, 101);
    println!("生成的数字是:{}", i);

    let mut guess = String::new();

    let _ = io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是:{}", guess);
    let i1 = guess.trim().parse::<i32>().expect("error");

    match i1.cmp(&i) {
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too small!"),
        Ordering::Less => println!("To bigger!")
    }
}
