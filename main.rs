use std::io;

fn read_int() -> i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    /*let mut name = String::new();
    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {}!", name.trim());

    

    println!{"Your Age: "};
    let age = read_int();
    println!("Age: {age}");

    if age >= 18{
        println!("Adult");
    } else {
        println!("Minor");
    }*/

    let mut count = 0;

    loop{
        println!("{count}");

        count += 1;

        if count == 5 {
            break;
        }
    };

    let result = loop{
        println!("hello");
        break 10;
    };
    println!("{result}");

    let mut count = 0;
    while count<=5{
        println!("Hello While");
        count += 1;
    }


    for i in 1..=5{
        println!("{i}")
    }
    
}