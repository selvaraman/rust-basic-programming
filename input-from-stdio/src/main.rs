use std::io;

fn main() {
    let mut input = String::new();
    println!("value of x:");
    io::stdin().read_line(&mut input).expect("Error getting value of x");
    let x = input.trim().parse::<i32>().unwrap();
    input.clear();
    println!("value of y:");
    io::stdin().read_line(&mut input).expect("Error getting value of y");
    let y = input.trim().parse::<i32>().unwrap();
    println!("x: {}", x);
    println!("y: {}", y);
    let z = x + y;
    println!("z: {}", z);
}
