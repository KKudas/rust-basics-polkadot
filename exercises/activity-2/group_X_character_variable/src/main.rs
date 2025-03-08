fn main() {
    let character = 'A';
    let mut name = String::from("Ralph");

    println!("Character: {}", character);
    println!("Original name: {}", name);
    name = format!("{}{}", character, &name[1..]);

    println!("Modified name: {}", name);
}

