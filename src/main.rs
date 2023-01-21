fn main() {
    println!("Hello, world!");
    // let number: u8 = 0;
    let input: String = String::new();
    std::io::stdin().read_line(input);
}

// pub mod std {
//     pub mod io {
//        pub fn stdin() {
//              ...
//         }
//     }
// }

// Compound Data-Types
// Structure
struct Person {
    weight: u16,
    height: f64,
    eye_color: String
}

impl Person {
    fn new() -> Person {
       return Person {
            weight: 0,
            height: 0,
            eye_color: ?
       }
    }
}

// trait PersonTraits {
//     fn think();
// }

// impl PersonTraits for Person {
//     fn think() {
//         println!("thinking...");
//     }
// }

// // Enumeration
// enum Creature {
//     Person,
//     Animal,
//     Plant
// }

// // Trait
// trait BeingAlive {
//     fn walk() {}
//     fn eat() {}
//     // ...
// }

// Person.weight;
// Creature::Person;