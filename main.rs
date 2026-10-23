use std::io;

fn read_int() -> i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {}!", name.trim());

    

    println!{"Your Age: "}
    let age = read_int();
    println!("Age: {age}")
    
}