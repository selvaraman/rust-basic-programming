use std::collections::HashMap;

fn main() {
    let str1: String = String::from("radar");
    let mut res: HashMap<char, u32> = HashMap::new();
    for ch in str1.chars() {
        res.entry(ch).and_modify(|count| *count+=1).or_insert(1);
    }

    for (key, val) in res {
        println!("{} - {}", key, val);
    }
}
