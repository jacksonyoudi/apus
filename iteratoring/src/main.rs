fn main() {
    println!("Hello, world!");

    let mut counter: Counter = Counter { count: 0 };
    println!("{}", counter.next().unwrap());
    println!("{}", counter.next().unwrap());
}


// trait InIterator<T: Copy> {
//     fn each<F: Fn(T) -> T>(&mut self, f: F);
// }
//
// impl<T: Copy> InIterator<T> for Vec<T> {
//     fn each<F: Fn(T) -> T>(&mut self, f: F) {
//         let mut i: i32 = 0;
//         let x: i32 = self.len().try_into().unwrap();
//         while i < x {
//             self[i] = f(self[i]);
//             i += 1;
//         }
//     }
// }


struct Counter {
    count: usize,
}


impl  Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}





