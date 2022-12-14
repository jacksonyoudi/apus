fn main() {
    let _i: i32 = 5;
    println!("Hello, world!");


//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e,..
// } = Struct { e: 5 };

    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);


    // 这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，
    // 而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。

    // 变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），
    // 就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。


    // let guess: i32 = ... 或者 "42".parse::<i32>()
    let number: i32 = "42".parse::<i32>().expect("not a nummer");

    // assert!(0.1 + 0.2 == 0.3);

    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 1..5 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }
    let x1: f64 = 13.14_f64.round();

    let x2: char = 'c';

    // 由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节：
    let x = '中';
    println!("占用多少{}内存", std::mem::size_of_val(&x));

    // Rust 的字符只能用 '' 来表示， "" 是留给字符串的


    let t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    // 单元类型 ()
    // 再比如，你可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key。
    // ZST
    // 这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存。


    // 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值，因此在上述函数体的三行代码中，前两行是语句，最后一行是表达式。
    //
    // 对于 Rust 语言而言，这种基于语句（statement）和表达式（expression）的方式是非常重要的，你需要能明确的区分这两个概念, 但是对
    // 于很多其它语言而言，这两个往往无需区分。基于表达式是函数式语言的重要特征，表达式总要返回值。


    //  函数要点
    //      函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
    //      函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
    //      每个函数参数都需要标注类型


    // 永不返回的发散函数 !

}


struct Struct {
    e: i32,
}