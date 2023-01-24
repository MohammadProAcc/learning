fn main() {
    println!("Hello, Welcome to the Guessing Game!");

    let random: u8 = random_number::random!(1..=100);
    // println!("random number is: {}", random);

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input);

    let parsed_input = input.parse::<u8>();

    let result = random.cmp(&parsed_input);
}