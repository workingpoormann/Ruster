use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
};

// Custom struct
struct Person {
    name: String,
    age: u8,
}

struct Point(i32, i32);

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // simple variant
    Run(Direction),              // tuple variant
    Teleport { x: u32, y: u32 }, // struct variant
}

enum CarryableConcreateItem {
    Left,
    Right,
}

type Item = CarryableConcreateItem;
type PlayerInventoyr = RwLock<Vec<Arc<RefCell<Item>>>>;

const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 42 } else { 13 }
}

static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    {
        println!("Hello, world!");

        let mut peter = Person {
            name: String::from("Peter"),
            age: 27,
        };

        // not move
        describe(&peter);

        // not move
        peter.age = 28;
        describe(&peter);

        let name = String::from("Avery");
        let age = 39;
        let avery = Person { name, age };
        describe(&avery);
    }

    {
        let p = Point(17, 23);
        println!("({}, {})", p.0, p.1);
    }

    {
        let dir = Direction::Left;
        println!("{dir:?}");

        let player_move: PlayerMove = PlayerMove::Run(dir);
        println!("On this turn: {player_move:?}");
    }

    {
        println!("{BANNER}");
    }
}
