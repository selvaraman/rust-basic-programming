fn get_sub_array_index(s: u32, arr: &[u32]) -> (usize, usize) {
    let (mut left_index, mut right_index, mut sum) = (0, 0, 0);
    for _ in arr {
        sum += arr[right_index];
        if sum == s {
            return (left_index, right_index);
        } else if sum > s {
            sum -= arr[left_index];
            left_index += 1;
            if sum == s {
                return (left_index, right_index);
            }
            right_index -= 1;
        } else {
            right_index += 1;
            if right_index == arr.len() {
                break;
            }
        }
    }
    (0, 0)
}

fn main() {
    let sum = 12;
    let arr: Vec<u32> = vec![1, 2, 3, 7, 5];
    //let sum = 15;
    //let arr: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Sum: {}\nArr: {:?}", sum, arr);
    let (left_index, right_index) = get_sub_array_index(sum, &arr);
    println!("{}, {}", left_index + 1, right_index + 1);
}
