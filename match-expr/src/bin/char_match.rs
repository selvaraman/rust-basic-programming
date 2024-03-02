fn main() {
    let c = 'A';
    match c {
        'a' => println!("first alpha"),
        'z' => println!("last char"),
        val if val.is_ascii_uppercase() => println!("sent upper case letter"),
        _ => println!("does not matching anything")
    }
}
