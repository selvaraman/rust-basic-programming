fn factorial(n: i32) -> i32 {
    if n == 1 || n == 0 {
        return 1;
    }
    n * factorial(n-1)
}

fn main() {
    let n = 10;
    println!("Factorial of {} is {}", n, factorial(n))
}
