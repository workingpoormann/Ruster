fn collatz_sequence(mut n: i32) -> u32 {
    let mut length: u32 = 1;

    while n > 1 {
        length += 1;
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
    }

    length
}

fn main() {
    println!("Hello, world!");

    for i in 1..=10 {
        println!("collatz({}) => Length: {}", i, collatz_sequence(i))
    }
}
