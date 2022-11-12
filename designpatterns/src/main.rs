mod blog;
mod graph;

// use crate::blog::post::Post;
use crate::graph::{Circle, CircleBuilder};

use std::process::Command;

fn main() {
    {
        // let mut post = Post::new();
        //
        // post.add_text("I ate a salad for lunch today");
        // assert_eq!("", post.content());
        //
        // post.request_review();
        // assert_eq!("", post.content());
        //
        // post.approve();
        // assert_eq!("I ate a salad for lunch today", post.content());
    }


    // 建造者模式
    let circle: Circle = Circle::new()
        .x(1.0)
        .y(2.0)
        .radius(2.0)
        .build();

    // assert_eq!(circle.area(), 12.5666);
    println!("{}", circle.area());
    assert_eq!(circle.x, 1.0);
    assert_eq!(circle.y, 2.0);

    // Command::new("ls")
    //     .args("-l")
    //     .args("-a")
    //     .spawn()
    //     .expect("ls command failed to start");



}