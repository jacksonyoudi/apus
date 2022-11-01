use std::fmt::{Debug, Display};
use std::ops::Deref;

#[warn(dead_code)]
#[warn(dead_code)]
fn main() {
    println!("Hello, world!");
    let one = Point {
        x: 5,
        y: 10,
    };

    let point = Point {
        x: 1.0,
        y: 2.0,
    };

    let x = point.x();
    println!("{}", x.deref());
    let line = Line {
        x: 1,
        y: 9.8,
    };

    let post = Post { title: "Rust语言简介".to_string(), author: "Sunface".to_string(), content: "Rust棒极了!".to_string() };
    let weibo = Weibo { username: "sunface".to_string(), content: "好像微博没Tweet好用".to_string() };

    println!("{}", post.summary());
    println!("{}", weibo.summary());

    notify(&post);

    let i: u16 = 100;
    let x1: i32 = i.try_into().unwrap();


    let i1: i32 = 5;
    let i2: Kilometers = 5;

    println!("{}", i1 + i2);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


struct Line<T, U> {
    x: T,
    y: U,
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr)
}


// Rust 通过在编译时进行泛型代码的 单态化(monomorphization)来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
//
// 编译器所做的工作正好与我们创建泛型函数的步骤相反，编译器寻找所有泛型代码被调用的位置并针对具体类型生成代码。


pub trait Summary {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
    fn summarize(&self) -> String {
        String::from("")
    }
}


pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summary(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    // fn summary(&self) -> String {
    //     format!("{}发表了微博{}", self.username, self.content)
    // }
}


pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


pub fn notify_one<T: Summary>(item: &T) {
    todo!()
}


pub fn notify_two<T: Summary + Display>(item: &T) {
    todo!()
}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    todo!()
}

pub fn some_function_one<T, U>(t: &T, u: &U)
    where T: Display + Clone,
          U: Clone + Debug
{
    todo!()
}

type Kilometers = i32;


trait Add<RHS, Output> {
    fn add(&self, rhs: RHS) -> Output;
}


impl Add<i32, i32> for i32 {
    fn add(self, rhs: i32) -> i32 {
        *self + rhs
    }
}


impl Add<u32, u32> for u32 {
    fn add(&self, rhs: u32) -> u32 {
        *self + rhs
    }
}


trait Page {
    fn set_page(&self, page: i32) {
        println!("{}", 1);
    }
}

trait PerPage {
    fn set_per_page(&self, num: i32) {
        println!("{}", 10)
    }
}


struct MyPaginate {
    page: i32,
}

impl Page for MyPaginate {}

impl PerPage for MyPaginate {}


trait Paginate: T
    where T: Page + PerPage
{
    fn set_skip_page(&self, num: i32) {
        println!("Skip Page {:?}", num)
    }
}

impl<T> Paginate for T
    where T: Page + PerPage
{}








