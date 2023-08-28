fn main() {
    let byte1: u8 = 72;
    let str1 =  format!("0X{:02X}", byte1);
    println!("{}", str1);
}
