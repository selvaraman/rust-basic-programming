use std::env;
use std::process;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 0 {
        println!("element is not provided");
        process::exit(1);
    }
    let x = args[0].parse::<u32>().unwrap_or(0);
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 | 5 => println!("four or five"), //multiple match pattern
        6..=10 => println!("six to ten"), //range pattern
        e @ 11..=20 => println!("got values 11 to 20 range and the value is {}", e), // Subpatterns - variable @ subpattern
        _ => println!("no match") //wildcard pattern
    }
}
