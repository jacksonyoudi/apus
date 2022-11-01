use macroadvance;

fn main() {
    let x = macroadvance::my_vec![1,2,3];
    //

    println!("{:?}", x);

    println!("Hello, world!");
}


fn first_word(s: &String) -> usize {
    let x: &[u8] = s.as_bytes();


    for (i, &item) in x.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// 单态化
// 静态分发
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}


// 单态化静态分发的好处是性能好，没有运行时开销；缺点是容易造成编译后生成的二进制文件膨胀。





