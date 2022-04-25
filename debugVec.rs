use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>);

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{:#?}", v)
}
