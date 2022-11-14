#[warn(unused_imports)]
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    {
        // let handle: JoinHandle<()> = thread::spawn(
        //     || {
        //         for i in 1..10 {
        //             println!("hi nummer: {} from the spawned thread!", i);
        //             thread::sleep(Duration::from_millis(1));
        //         }
        //     }
        // );
        //
        // for i in 0..5 {
        //     println!("hi nummer: {} from the main thread!", i);
        //     thread::sleep(Duration::from_millis(1));
        // }
        //
        // handle.join().unwrap();
    }

    let x: PrintDrop = PrintDrop("x");
    let y: PrintDrop = PrintDrop("y");

    let mut v = vec![];

    for id in 1..5 {
        let child = thread::spawn(move || {
            println!("in child: {}", id);
        });
        v.push(child)
    }

    println!("in main: join before");
    for child in v {
        child.join();
    }

    println!("in main: join after")
}

//  NewType类型
// 使复制语义的类型具有移动语义。
// 比如 f64 本来是复制语义，而包装为 Miles（f64）之后，因为结构体本身不能被自动实现Copy，所以Miles（f64）就成了移动语义。
struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0)
    }
}









