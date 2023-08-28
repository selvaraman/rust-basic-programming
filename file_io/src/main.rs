use std::fs;

fn main() {
    let content = fs::read_to_string("./sample.txt");
    println!("{:?}", content.unwrap());
}
