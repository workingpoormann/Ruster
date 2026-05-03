fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (left, middle, right) = tuple;
    left < middle && middle < right
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }

    result
}

fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 10;
    println!("a: {a:?}");

    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);

    let primes = [2, 3, 5, 7, 11, 13, 17, 19];

    for prime in primes {
        dbg!(prime);
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    let tuple = (1, 5, 3);
    println!(
        "{tuple:?}? {}",
        if check_order(tuple) {
            "ordered"
        } else {
            "unordered"
        }
    );

    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("Original: ");
    for row in matrix {
        println!("{row:?}")
    }

    let transposed = transpose(matrix);

    println!("\nTransposed");
    for row in transposed {
        println!("{row:?}");
    }
}
