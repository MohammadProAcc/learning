fn main() {
    // println!("Hello, Welcome to the Guessing Game!");

    // let mut input: String = String::new();
    // std::io::stdin().read_line(&mut input);

    // println!("you entered >> {}", input);
    // println!("{:?}", Type::Variant_1);
    let number = 1276;
    let person_1 = Person {
        name: String::from("mohammad hossein"),
        weight: 60,
        height: 180
    };

    // println!( "Hello, the number is {:b}", number);
    // println!("Hello, i'm mohammad hossein, my weight is {}", person_1.weight);
    println!("{}", person_1.name);
}

struct Person {
    name: String,
    weight: u8,
    height: u8,
}

impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}'s characteristics: \nweight: {}\nheight: {}", self.name, self.weight, self.height)
    }
}

// enum Type {
//     Variant_1,
//     Variant_2
// }