use std::sync::{Arc, Condvar, Mutex, MutexGuard, RwLock, RwLockWriteGuard};
use std::thread;

mod atomicdemo;

fn main() {
    println!("Hello, world!");


    let lock = RwLock::new(5);

    {
        // 引用
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();

        assert_eq!(*r1, 5);

        assert_eq!(*r2, 5);
    }

    {
        let mut w: RwLockWriteGuard<i32> = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }

    // 互斥锁 与 条件变量
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone: Arc<(Mutex<bool>, Condvar)> = pair.clone();

    thread::spawn(
        move || {
            let &(ref lock, ref cvar): &(Mutex<bool>, Condvar) = &*pair_clone;
            let mut started: MutexGuard<bool> = lock.lock().unwrap();

            *started = true;
            cvar.notify_one();
        });


    let &(ref lock, ref cvar): &(Mutex<bool>, Condvar) = &*pair;
    let mut started: MutexGuard<bool> = lock.lock().unwrap();

    while !*started {
        println!("{}", started);
        started = cvar.wait(started).unwrap();

        println!("{}", started);
    }

    atomic_t();
}


