#[derive(Debug, PartialEq)]
enum X<'a> {
    Y(&'a str, &'a str)
}

const ARR: [X; 4] = [
    X::Y("pdf", "application/pdf"),
    X::Y("gif", "image/gif"),
    X::Y("avif", "image/avif"),
    X::Y("png", "image/png")
];

fn main() {
    let a = X::Y("pdf", "application/pdf");
    println!("Result: {:?}", a);
    println!("{:?}", ARR);
    let res = ARR.iter().any(|e| e == &a);
    println!("Result: {:?}", res);
}
