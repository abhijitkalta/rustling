use std::fmt; // Import format

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement 'Display' for 'MinMax'
impl  fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl  fmt::Display for Point2D {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "x: {}, y: {}", self.x, self.y) 
   } 
}


fn main() {
    let minmax = MinMax(0, 10);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-500, 500);
    let small_range = MinMax(-5, 5);

    println!(
        "The big range is {big} and small range is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}
