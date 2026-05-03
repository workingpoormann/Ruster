fn main() {
    // while
    {
        let mut x = 200;

        while x >= 100 {
            x = x / 2;
        }

        dbg!(x);
    }

    // for
    {
        for x in 1..5 {
            print!("_{}", x);
        }
        println!();

        for elem in [2, 4, 6, 8, 10] {
            print!(" {}", elem);
        }
        println!();
    }

    // loop
    {
        let mut i = 0;
        loop {
            i += 1;
            print!(" {}", i);

            if i > 100 {
                break;
            }

            if i == 50 {
                i += 30;
                continue;
            }
        }
        println!();
    }

    // labels
    {
        let s = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        'outer: for i in 0..=2 {
            print!(" *{}", i);
            if s[i] == 8 {
                break 'outer;
            }
        }
        println!();
    }
}
