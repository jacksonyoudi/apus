#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};

pub fn generator_test() {
    let mut gen = move || {
        yield 1;
        yield 2;
        yield 3;
        yield 4;
    };

    unsafe {
        for _ in 0..4 {
            let c: GeneratorState<i32, ()> = gen.resume();
            println!("{:?}", c);
        }
    }
}