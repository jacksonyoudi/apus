use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
    let list: List = Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(Nil))
        ))
    ));


    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x1: Box<i32> = Box::new(x);

    assert_eq!(5, *x1);

    let my_box: MyBox<i32> = MyBox::new(6);

    assert_eq!(6, *my_box)
}