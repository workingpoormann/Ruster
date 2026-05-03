// functions
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 { gcd(b, a % b) } else { a }
}

/*
    Macros
    Macros are expanded into Rust code during compilation,
    and can take a variable number of arguments.

    They are distinguished by a ! at the end.

    The Rust standard library includes an assortment of useful macros.
*/
fn factorial(n: u32) -> u32 {
    let mut product = 1;

    for i in 1..=n {
        // dbg! logs the value of the expression and returns it.
        product *= dbg!(i);
    }

    product
}

fn fizzbuzz(n: u32) -> u32 {
    if n % 15 == 0 {
        dbg!("fizzbuzz");
        0
    } else if n % 3 == 0 {
        dbg!("fizz");
        0
    } else if n % 5 == 0 {
        dbg!("buzz");
        0
    } else {
        n
    }
}

fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;

    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }

    len
}

fn main() {
    {
        println!("my gcd");

        let share_mod = gcd(143, 52);
        println!("share mod: {share_mod}");
    }

    {
        println!("factorial");

        let f = factorial(5);
        println!("factorial = {f}");

        /*
        for i in 1..=15 {
            println!("i : {}", fizzbuzz(i));
        }
        */

        println!("collatz length: {}", collatz_length(11));
    }
}
