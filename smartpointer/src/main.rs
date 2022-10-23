use std::cell::RefCell;
use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::ops::Drop;
use std::mem::drop;
use std::rc::{Rc, Weak};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
//
// struct MyBox<T>(T);
//
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
//
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// fn hello(name: &str) {
//     println!("hello: {}", name);
// }
//
//
// // String  vec
// struct CustomSmartPointer {
//     data: String,
// }
//
//
// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data: {}", self.data);
//     }
// }
//
//

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    // let list: List = Cons(1, Box::new(
    //     Cons(2, Box::new(
    //         Cons(3, Box::new(Nil))
    //     ))
    // ));
    //
    //
    // let x: i32 = 5;
    // let y: &i32 = &x;
    //
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    //
    // let x1: Box<i32> = Box::new(x);
    //
    // assert_eq!(5, *x1);
    //
    // let my_box: MyBox<i32> = MyBox::new(6);
    //
    // assert_eq!(6, *my_box);
    // assert_eq!(6, *(my_box.deref()));
    //
    //
    // let my_box1 = MyBox::new(String::from("Rust"));
    //
    // hello(&my_box1);
    //
    //
    // let pointer: CustomSmartPointer = CustomSmartPointer { data: String::from("my stuff") };
    // let pointer1: CustomSmartPointer = CustomSmartPointer { data: String::from("other stuff") };
    // drop(pointer);
    //
    // println!("end");


    let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


    let leaf: Rc<Node> = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch: Rc<Node> = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf branch = {:?}",leaf.parent.borrow().upgrade())

}


