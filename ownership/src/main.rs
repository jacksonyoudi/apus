use macroadvance;
use std::ops::Drop;

fn main() {
    let x = macroadvance::my_vec![1,2,3];
    //

    println!("{:?}", x);

    println!("Hello, world!");

    let s: S = S(1);
    println!("{:?}", s);

    let s: S = S(2);
    println!("{:?}", s);

    let a: A = A { a: 1, b: 2 };
    let b: A = a;

    println!("{:?}", a);

    let (a, b): (i32, i32) = (2, 3);

    assert_eq!(math(sum, a, b), 5);

    let x1: fn() = hello;
    println!("{:p}", x1);
}


fn first_word(s: &String) -> usize {
    let x: &[u8] = s.as_bytes();


    for (i, &item) in x.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// 单态化
// 静态分发
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}


// 单态化静态分发的好处是性能好，没有运行时开销；缺点是容易造成编译后生成的二进制文件膨胀。


// SBRM 作用域界定资源管理
#[derive(Debug)]
struct S(i32);

impl Drop for S {
    fn drop(&mut self) {
        println!("drop {}", self.0)
    }
}


#[derive(Debug, Copy, Clone)]
struct A {
    a: u32,
    b: u32,
}


// 函数 gcd 使用欧几里得算法（辗转相除法）求两数中的最大公约数。
// 如果a%b的余数不为0，则将b和a相互置换，将余数作为b的值，
// 继续递归求值；如果余数为0，则提前返回a。
// 其实此例中如果gcd函数使用if-else条件分支，阅读性会更好一些
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}


fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}


// 函数指针
fn hello() {
    println!("hello");
}











