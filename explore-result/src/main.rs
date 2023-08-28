fn is_even(num: u32) -> Result<bool, String> {
    if num % 2 == 0 {
        Ok(true)
    } else {
        Err("it is not even".to_owned())
    }
}

fn is_divisible_by_4(num: u32) -> Result<bool, String> {
    let even_result = is_even(num).unwrap_or_else(|err| Err(err));
    if even_result.is_err() {
        return even_result;
    }
    
    if num % 4 == 0 {
        Ok(true)
    } else {
        Err("it is not divisible by 4".to_owned())
    }
}

fn main() {
    let x: u32 = 20;
    println!("{:?}", is_even(x));
}
