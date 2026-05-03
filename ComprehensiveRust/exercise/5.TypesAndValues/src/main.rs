fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c + a;
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    println!("Hello, world!");

    {
        let x: i32 = 10;
        println!("x: {x}");

        let b: bool = true;
        println!("b : {b}");

        println!("result: {}", interproduct(10, 20, 30));
        let f: u32 = 50;
        println!("fib({f}): {}", fib(f));
    }
}
