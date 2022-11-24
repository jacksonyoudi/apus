use std::any::{Any, TypeId};

enum E { H, He, Li }

struct S {
    x: u8,
    y: u8,
    z: u16,
}


fn main() {
    let v1: u32 = 0xc0ffee_u32;
    let v2: E = E::He;
    let v3: S = S {
        x: 0xde,
        y: 0xad,
        z: 0xbeef,
    };
    let v4: &str = "rust";
    let mut a: &dyn Any;
    a = &v1;
    assert!(a.is::<u32>());
    println!("{:?}", TypeId::of::<u32>());
    a= &v2;
    assert!(a.is::<E>());
    println!("{:?}", TypeId::of::<E>());

    a= &v3;
    assert!(a.is::<S>());
    println!("{:?}", TypeId::of::<S>());

    a= &v4;
    assert!(a.is::<&str>());
    println!("{:?}", TypeId::of::<&str>());
}
