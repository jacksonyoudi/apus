## 什么是字符串?
    顾名思义，字符串是由字符组成的连续集合，但是在上一节中我们提到过，
    Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，
    字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)，这样有助于大幅降低字符串所占用的内存空间。

Rust 在语言级别，只有一种字符串类型： str，它通常是以引用类型出现 &str，也就是上文提到的字符串切片。
虽然语言级别只有上述的 str 类型，但是在标准库里，还有多种不同用途的字符串类型，其中使用最广的即是 String 类型。

str 类型是硬编码进可执行文件，也无法被修改，
但是 String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串，当 Rust 用户提到字符串时，
往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。

除了 String 类型的字符串，Rust 的标准库还提供了其他类型的字符串，
例如 OsString， OsStr， CsString 和 CsStr 等，注意到这些名字都以 String 或者 Str 结尾了吗？
它们分别对应的是具有所有权和被借用的变量。


## String 与 &str 的转换

```rust
    let s1: String = "test".to_string();
    let s2: &str = s1.as_str();
```


## 创建结构体实例

```rust

#![allow(unused)]
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

有几点值得注意:

初始化实例时，每个字段都需要进行初始化
初始化时的字段顺序不需要和结构体定义时的顺序一致


## 元组结构体(Tuple Struct)

结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体

## 单元结构体(Unit-like Struct)

还记得之前讲过的基本没啥用的单元类型吧？单元结构体就跟它很像，没有任何字段和属性，但是好在，它还挺有用。

如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体：

```rust

#![allow(unused)]
fn main() {
struct AlwaysEqual;

let subject = AlwaysEqual;

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
impl SomeTrait for AlwaysEqual {

}
}

```


## dbg!


## 枚举
    我们对之前的 枚举类型 和 枚举值 进行了重点标注，这是因为对于新人来说容易混淆相应的概念，
    总而言之： 枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。


```rust
enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}

let heart = PokerSuit::Hearts;
let diamond = PokerSuit::Diamonds;




```



### for 循环



| 使用方法|	等价使用方式| 	所有权   |
|---|---|--------|
|for item in collection	|for item in IntoIterator::into_iter(collection) | 	转移所有权 |
|for item in &collection|	for item in collection.iter()	| 不可变借用  |
|for item in &mut collection|	for item in collection.iter_mut()	| 可变借用   |



### match 匹配

```rust
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}

```

`match` 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
`match` 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
`X | Y`，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可




