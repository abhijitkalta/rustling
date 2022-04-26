use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter a number index: ");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Not a number");

    let element = a[index];
    println!(
        "The value of the element at index {} is : {}",
        index, element
    );

    for number in (1..4).rev() {
        print!("{}", number);
    }
}
