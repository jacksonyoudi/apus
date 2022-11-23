/*!
CSP（Communicating Sequential Processes，通信顺序进程）是一个精确描述并发的数学理论，
基于该理论构建的并发程序不会出现常见的问题，并且可以得到数学证明。
CSP对程序中每个阶段所包含对象的行为进行精确的指定和验证，它对并发程序的设计影响深远
 */
use std::sync::mpsc::{channel, Receiver, Sender, sync_channel, SyncSender};
use std::thread;

fn main() {
    println!("Hello, world!");


    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    for i in 0..10 {
        let tx: Sender<i32> = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j: i32 = rx.recv().unwrap();
        assert!(0 <= j && j < 10);
    }

    let (tx, rx): (SyncSender<i32>, Receiver<i32>) = sync_channel(1);
    tx.send(1).unwrap();

    thread::spawn(move || {
        tx.send(2).unwrap();
    });

    assert_eq!(rx.recv().unwrap(), 1);
    assert_eq!(rx.recv().unwrap(), 2);
}
