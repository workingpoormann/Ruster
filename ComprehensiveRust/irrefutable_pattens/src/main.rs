use std::time::Duration;

fn takes_tuple(tuple: (char, i32, bool)) {
    let _a = tuple.0;
    let _b = tuple.1;
    let _c = tuple.2;

    let (_a, _b, _c) = tuple;
    let (_, _b, _c) = tuple;
    let (.., _c) = tuple;
    println!("{} {} {}", _a, _b, _c);
}

fn matching_values() {
    let input = 'x';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving Around"),
        '0'..='9' => println!("Number input!"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Something else"),
    }
}

struct Move {
    delta: (i32, i32),
    repeat: u32,
}

#[rustfmt::skip]
fn destruct_by_matching() {
    let m = Move {delta: (10, 0), repeat: 5};

    match m {
        Move { delta: (0, 0), .. } => println!("Standing still"),
        Move {delta : (x, 0), repeat} => println!("{repeat} step x: {x}"),
        Move { delta: (0, y), repeat: 1} => println!("Single step y: {y}"),
        _ => println!("Other move"),
    }
}

enum MyResult {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> MyResult {
    if n % 2 == 0 {
        MyResult::Ok(n / 2)
    } else {
        MyResult::Err(format!("cannot divide {n} into two equal parts"))
    }
}

/*
    Like tuples, enums can also be destructured by matching:
    Patterns can also be used to bind variables to parts of your values.
    This is how you inspect the structure of your types.
    Let us start with a simple enum type:*
*/
fn enums() {
    let n = 100;

    // pattern matching.
    match divide_in_two(n) {
        MyResult::Ok(half) => println!("{n} divided in two is {half}"),
        MyResult::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}

fn sleep_for(secs: f32) {
    let result = Duration::try_from_secs_f32(secs);

    if let Ok(duration) = result {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    }
}

// The if let expression lets you
// execute different code depending on whether a value matches a pattern:
fn let_expressions() {
    sleep_for(-2.0);
    sleep_for(0.8);
}

/*
    Like with if let,
    there is a while let variant
    that repeatedly tests a value against a pattern:*

    Here String::pop returns Some(c) until the string is empty,
    after which it will return None.
    The while let lets us keep iterating through all items.
*/
fn while_let_statements() {
    let mut name = String::from("Comprehensive Rust");

    while let Some(c) = name.pop() {
        print!("{}", c)
    }
    println!();
}

/*
    For the common case of matching a pattern and returning from the function,
    use let else.
    The “else” case must diverge
    (return, break, or panic - anything but falling off the end of the block).
*/
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("got None"));
    };

    let first_byte_char = if let Some(first) = s.chars().next() {
        first
    } else {
        return Err(String::from("got empty string:"));
    };

    let digit = if let Some(digit) = first_byte_char.to_digit(16) {
        digit
    } else {
        return Err(String::from("not a hex digit."));
    };

    Ok(digit)
}

fn main() {
    takes_tuple(('a', 777, true));
    matching_values();
    destruct_by_matching();
    enums();
    let_expressions();
    while_let_statements();
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
