fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut tranposed: [[i32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    for i in 0..=2 {
        for j in 0..=2 {
            tranposed[j][i] = matrix[i][j];
        }
    }

    tranposed
}

fn main() {
    println!("Hello, world!");

    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("* Original:");
    for row in matrix {
        println!("{row:?}");
    }

    let transposed = transpose(matrix);
    println!("\n* Transposed:");
    for row in transposed {
        println!("{row:?}");
    }
}
