use std::str::FromStr;
use std::env;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
                3 * 11);
}

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
        // expect returns the value if the parse is successful, else an error message is returned
    }

    if numbers.len() == 0 {
        eprintln!("Usage: cargo run <NUM1> <NUM2>");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {  // m borrows a reference to the elements in numbers
        d = gcd(d, *m);  // dereference the value in m
    }

    println!("The greatest common divisor of {:?} is {}",
             numbers, d);
}

