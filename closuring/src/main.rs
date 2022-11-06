#![feature(unboxed_closures, fn_traits)]
// 定义的结构体 其中的成员变量 用于捕获外部变量
struct MyClosure {
    env_var: u32,
}


// move 语义
impl FnOnce<()> for MyClosure {
    type Output = u32;

    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        println!("call it FnOnce()");
        self.env_var + 2
    }
}


// 可变借用
impl FnMut<()> for MyClosure {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> Self::Output {
        println!("call it FnMut()");
        self.env_var + 2
    }
}


// 不可变 借用
impl Fn<()> for MyClosure {
    extern "rust-call" fn call(&self, args: ()) -> Self::Output {
        println!("call it Fn()");
        self.env_var + 2
    }
}

fn call_it<F: Fn() -> u32>(f: &F) -> u32 {
    f()
}

fn call_it_mut<F: FnMut() -> u32>(f: &mut F) -> u32 {
    f()
}

fn call_it_once<F: FnOnce() -> u32>(f: F) -> u32 {
    f()
}


fn main() {
    // found closure `[closure@closuring/src/main.rs:2:17: 2:48]`
    let _c: fn() = || { println!("hello, world") };


    let env_var = 1;
    let mut c = MyClosure { env_var: env_var };

    c();
    c.call();
    c.call_mut();
    c.call_once();

    let mut c = MyClosure { env_var: env_var };
    {
        assert_eq!(3, call_it(&c));
    }
    {
        assert_eq!(3, call_it_mut(&mut c));
    }
    {
        assert_eq!(3, call_it_once(c));
    }
}

