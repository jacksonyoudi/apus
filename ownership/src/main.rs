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