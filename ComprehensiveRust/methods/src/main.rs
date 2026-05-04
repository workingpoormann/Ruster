/*
Rust allows you to associate functions with your new types.
You do this with an impl block:

The self arguments specify the “receiver” - the object the method acts on.
There are several common receivers for a method:

&self: borrows the object from the caller
    using a shared and immutable reference.
    The object can be used again afterwards.

&mut self: borrows the object from the caller
    using a unique and mutable reference.
    The object can be used again afterwards.

self: takes ownership of the object and moves it away from the caller.
    The method becomes the owner of the object.

    The object will be dropped (deallocated) when the method returns,
    unless its ownership is explicitly transmitted.
    Complete ownership does not automatically mean mutability.

mut self: same as above, but the method can mutate the object.

No receiver: this becomes a static method on the struct.
    Typically used to create constructors that are called new by convention.
*/
#[derive(Debug)]
struct CarRace {
    name: String,
    laps: Vec<i32>,
}

impl CarRace {
    // No receiver, a static method
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            laps: Vec::new(),
        }
    }

    // &mut self: borrows the object from the caller using a unique
    //      and mutable reference.
    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self (covered later)
    // takes ownership
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = CarRace::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();

    race.add_lap(71);
    race.print_laps();

    race.finish();
    // race.add_lap(42);
}
