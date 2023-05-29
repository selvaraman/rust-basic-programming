fn main() {
    let mut arr:Vec<i32> = vec![1, 2, 3, 4, 5];
    let length:usize = arr.len();
    println!("Array Before Reverse: {:?}", arr);
    for i in 0..length/2 {
        arr.swap(i, length-1-i);
    }
    println!("Array After Reverse: {:?}", arr);
}
