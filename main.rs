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

    /*
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
    */

    //println!("{}", calculate(1,2));

    //_tuple();

    //_tuple_extract();

    //_array_extract();

    //println!("{:#?}",_vec());

    _strings();
    
}

fn calculate(a: i32, b: i32) -> i32{
    a + b
}

fn _tuple() -> (&'static str, i32, f64){
    let user = ("Nikki", 22, 60.5);

    println!("{}", user.0);
    println!("{}", user.1);
    println!("{}", user.2);

    user
}

fn _tuple_extract(){
    let (name, age, weight) = _tuple();

    println!("{name}, {age}, {weight}");
}

fn _array()->[i32; 5]{
    let numbers = [10, 20, 30, 40, 50];

    println!("{}", numbers[0]);
    println!("{}", numbers[2]);

    println!("Length: {}", numbers.len());

    numbers
}

fn _array_extract(){
    let [a, b, c, d, e] = _array();
    println!("{a}");
}

fn _vec() -> Vec<i32> {
    let mut numbers = vec![10, 20, 30];

    numbers.push(40);
    numbers.push(50);

    numbers.pop();

    println!("{:?}", numbers);

    let a = numbers[2];
    println!("{:?}", numbers);

    numbers

}


fn _strings(){
    let mut name = String::from("Nikki");
    name += " Singh";
    println!("{}", name);

}