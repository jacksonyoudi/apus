use std::sync::{Arc, Barrier};
use std::thread;
use std::thread::JoinHandle;

const MAX_POINTS: i32 = 100_000;

fn main() {
    println!("{}", MAX_POINTS);
    println!("Hello, world!");
    let x: u32 = "42".parse().expect("Not a number!!");
    let i: i32 = "42".parse::<i32>().expect("not a number");
    let x1: (i32, f64, i32) = (500, 6.4, 1);
    println!("{} {} {}", x1.0, x1.1, x1.2);

    let x2: [i32; 5] = [3; 5];

    println!("{}", x2[0]);
    println!("{}", x2[1]);
    println!("{}", x2.get(1).unwrap());

    antother_functions();
    let x3: [i32; 5] = [1, 2, 3, 4, 5];

    for x in x3.iter() {
        println!("{}", x);
    }

    for x in 1..=5 {
        println!("{}", x);
    }

    let x4: Vec<u32> = vec![1, 2, 3];
    let x5: bool = x4.any(|x| x == 3);

    println!("{}", x5);


    let mut handle: Vec<JoinHandle<()>> = Vec::with_capacity(5);
    let barrier: Arc<Barrier> = Arc::new(std::sync::Barrier::new(5));


    for _ in 0..5 {
        let c: Arc<Barrier> = barrier.clone();
        handle.push(thread::spawn(move || {
            println!("before wait");
            println!("rc:{}", );
            // 阻塞 直到 barrier.clone() 是5
            c.wait();
            println!("after wait");
        }));
    }
    for x in handle {
        x.join().unwrap();
    }
}

fn antother_functions() {
    println!("antother_functions")
}

// parameters 形参, arguments 实参
// 语句与表达式 (statement, )
// 语句没有返回值
// 返回值， 不能为返回值命名  表达式

// 注释
// loop


trait Any {
    fn any<F>(&self, f: F) -> bool
        where Self: Sized,
              F: Fn(u32) -> bool;
}

impl Any for Vec<u32> {
    fn any<F>(&self, f: F) -> bool where Self: Sized, F: Fn(u32) -> bool {
        for &x in self {
            if f(x) {
                return true;
            }
        }
        false
    }
}
