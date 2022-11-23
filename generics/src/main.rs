mod super_player;

fn give_me<T>(value: T) {
    let _: T = value;
}

fn main() {
    println!("Hello, world!");
    let x: &str = "gen";
    let y: i32 = 1024;
    give_me(x);
    give_me(y);


    let vec: Vec<u8> = Vec::<u8>::new();
}


enum Transmission<T> {
    Signal(T),
    NoSignal,
}


struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}