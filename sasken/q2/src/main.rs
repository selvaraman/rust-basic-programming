use std::collections::HashMap;

fn display_result(hash_map: HashMap<char, u32>) {
    for (key, val) in hash_map {
        println!("{} - {}", key, val);
    }
}

fn approach1(str1: String, str2: String) -> HashMap<char, u32> {
    let mut res: HashMap<char, u32> = HashMap::new();
    let mut final_result: HashMap<char, u32> = HashMap::new();
    let (long_str, short_str) = if str1.len() < str2.len() {
        (str2, str1)
    } else {
        (str1, str2)
    };
    for ch in long_str.chars() {
        res.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }

    for ch in short_str.chars() {
        if res.contains_key(&ch) {
            final_result
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(*res.get(&ch).unwrap() + 1);
        }
    }
    final_result
}

fn main() {
    let str1: String = String::from("selvamkkk");
    let str2: String = String::from("sasken");
    println!("{}", str1);
    println!("{}", str2);
    let final_result1 = approach1(str1, str2);
    display_result(final_result1);
}
