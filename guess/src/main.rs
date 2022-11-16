use std::cmp::Ordering;
use std::{io, thread};
use std::time::Duration;
// prelude
use rand::Rng; // trait

fn main() {
    {
        // let i: i32 = rand::thread_rng().gen_range(1, 101);
        // println!("生成的数字是:{}", i);


        // loop {
        //     println!("猜测一个数");
        //     let mut guess: String = String::new();
        //
        //     let _ = io::stdin().read_line(&mut guess).expect("无法读取行");
        //
        //     println!("你猜测的数是:{}", guess);
        //
        //     let i1: i32 = match guess.trim().parse::<i32>() {
        //         Ok(num) => num,
        //         Err(_) => continue,
        //     };
        //
        //
        //     match i1.cmp(&i) {
        //         Ordering::Equal =>
        //             {
        //                 println!("You Win!");
        //                 break;
        //             }
        //         Ordering::Greater => println!("Too Bigger!"),
        //         Ordering::Less => println!("To Smaller!")
        //     }
        // }
    }

    let park_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            // 阻塞
            thread::park();

            // thread::yield_now();

            println!("Thread unparked");
        }).unwrap();

    thread::sleep(Duration::from_millis(10));

    println!("unpark the thread");
    // 线程重启
    park_thread.thread().unpark();
    park_thread.join().unwrap();



    


}

