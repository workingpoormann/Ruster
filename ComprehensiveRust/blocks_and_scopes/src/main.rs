fn main() {
    {
        let z = 12;

        /*
         * The final expression of a block determines
         * the value and type of the whole block.
         */
        let x = {
            let y = 10;
            dbg!(y);
            z - y // 12 - 10
        };

        dbg!(x);
    }

    // You can use if as an expression.
    // The last expression of each block becomes the value
    // of the if expression.
    {
        let x: i8 = 10;

        // wow so weird D:
        let size = if x < 20 { "small" } else { "large" };
        println!("number size : {}", size);
    }

    // match Expressions
    {
        let val = 1;
        match val {
            1 => println!("one"),
            10 => println!("ten"),
            100 => println!("one hundred"),
            // default
            _ => {
                println!("something else");
            }
        }
    }

    {
        let flag = true;

        // and match can also return value...
        let val = match flag {
            true => 1,
            false => 0,
        };
        println!("The value of {flag} is {val}")
    }
}
