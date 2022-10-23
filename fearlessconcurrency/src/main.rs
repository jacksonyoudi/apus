#[warn(unused_imports)]
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let handle: JoinHandle<()> = thread::spawn(
        || {
            for i in 1..10 {
                println!("hi nummer: {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }
    );

    for i in 0..5 {
        println!("hi nummer: {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
