use random_number::random;
use std::cmp::Ordering;

fn main() {
    println!("Hello, Welcome to the Guessing Game!");

    let random: u8 = random!(1..=100);
    println!("{}", random);

    loop {

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input);

        let parsed_input: u8 = match input.trim().parse() {
            Result::Ok(x) => {
                println!("you entered: {}", x);
                x
            },
            Result::Err(msg) => {
                println!("sorry, you entered the arcane zone..\n{}", msg);
                panic!();
            }
        };

        match random.cmp(&parsed_input) {
            Ordering::Greater => {
                println!("the guess is too Low...");
            },
            Ordering::Less => {
                println!("the guess is too High...");
            }
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        };
    }

    // let number:u8 = 255;
    // match number {
    //     0..=100 => {
    //         println!("the number is between 1 till 99");
    //     } //     101..=200 => { //         println!("the number is between 100 till 199");
    //     }
    //     201..=255 => {
    //         println!("the number is out of range");
    //     }
    // }
}