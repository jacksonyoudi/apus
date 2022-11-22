use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::thread;

fn atomic_t() {
    let spinlock: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));
    let spinlock_clone: Arc<AtomicUsize> = spinlock.clone();

    thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    while spinlock.load(Ordering::SeqCst) != 0 {}


    // if let Err(panic) = thread::join() {
    //     println!("Thread had an error {:?}", panic);
    // }


    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(10).unwrap();
    });


    assert_eq!(rx.recv().unwrap(), 10);
}