fn main() {
    let mut source = vec![67, 34, 78, 12, 45, 70];
    println!("Before Sort: {:?}", source);
    for i in 0..source.len()-1 {
        let mut sorted = true;
        for j in 0..source.len()-1-i {
            if source[j] > source[j+1] {
                source.swap(j, j+1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
    println!("After Sort: {:?}", source);
}
