fn main() {
    let mut a = 5;
    println!("The value of a is: {}", a);
    a = 6;
    println!("The value of a is: {}", a);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, _z) = tup;
    println!("The value of y is: {}", y);

    another_function(x);

    let w = five();
    println!("The value of w is: {}", w);
}

fn another_function(input: i32) {
    println!("The value of x is {}", input);
}

fn five() -> i32 {
    5
}