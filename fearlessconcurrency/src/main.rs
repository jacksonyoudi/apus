extern crate core;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::panic;
#[warn(unused_imports)]
use std::thread;
use std::thread::{Builder, current, JoinHandle};
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

    {
        // let x: PrintDrop = PrintDrop("x");
        // let y: PrintDrop = PrintDrop("y");
        //
        // let mut v = vec![];
        //
        // for id in 1..5 {
        //     let child = thread::spawn(move || {
        //         println!("in child: {}", id);
        //     });
        //     v.push(child)
        // }
        //
        // println!("in main: join before");
        // for child in v {
        //     child.join();
        // }
        //
        // println!("in main: join after")
    }


    let mut v = vec![];
    for id in 1..5 {
        let thread_name: String = format!("child-{}", id);
        let size: usize = 3 * 1024;

        let builder: Builder = Builder::new()
            .name(thread_name)
            .stack_size(size);
        let child = builder.spawn(move || {
            println!("in child: {}", id);
            if id == 3 {
                panic::catch_unwind(|| {
                    panic!("oh no!")
                });
                println!("in {} do sm", current().name().unwrap())
            }
        }).unwrap();
        v.push(child);
    }
    for child in v {
        child.join().unwrap();
    }


    thread_local! {
        static FOO: RefCell<u32> = RefCell::new(1);
    }

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    thread::spawn(|| {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

//  NewType??????
// ?????????????????????????????????????????????
// ?????? f64 ???????????????????????????????????? Miles???f64??????????????????????????????????????????????????????Copy?????????Miles???f64???????????????????????????
struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0)
    }
}









