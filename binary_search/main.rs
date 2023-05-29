fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut l: usize = 0;
    let mut r: usize = arr.len() - 1;
    let seek: i32 = 4;
    let mut found: bool = false;
    while l != r {
        let mid: usize = (l + r) / 2;
        if arr[mid] == seek {
            println!("Number found at {:?}", mid);
            found = true;
            break;
        } else if arr[mid] < seek {
            l = mid;
        } else {
            r = mid;
        }
    }
    if !found {
        println!("Number not found");
    }
}
