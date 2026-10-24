use std::io;
use std::collections::HashMap;

fn _read_int() -> i32{
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

    //_strings();


    //_string_exp();
    //_hash();
    //_struct();
    _print_status(_Status::_Success);
}

enum _Status{
    _Pending,
    _Success,
    _Failed
}

fn _print_status(status: _Status){
    match status{
        _Status::_Pending => println!("Pending"),
        _Status::_Success => println!("Success"),
        _Status::_Failed => println!("Failed")

    }
}


fn _struct(){
    let a = _User{
        _name: String::from("Nikki"),
        _age: 22,
        _role: String::from("AI Engineer")
    };

    a._introduce();
    _try_user(&a);
}



#[derive(Debug)]
struct _User{
    _name: String,
    _age: u32,
    _role: String
}

impl _User {
    fn _introduce(&self) {
        println!("I'm {}, age {}", self._name, self._age);
    }
}

fn _try_user(_user: &_User){
    println!("I'm {}, age {}", _user._name, _user._age)
}

fn _hash(){
    let mut map = HashMap::new();

    map.insert("name", "Nikki");
    map.insert("city", "Delhi");
    map.insert("name", "Nikki2");

    println!("{:?}", map.get("name"));
    println!("{:?}", map.keys());
}
















fn _calculate(a: i32, b: i32) -> i32{
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
    let [a, _b, _c, _d, _e] = _array();
    println!("{a}");
}

fn _vec() -> Vec<i32> {
    let mut numbers = vec![10, 20, 30];

    numbers.push(40);
    numbers.push(50);

    numbers.pop();

    println!("{:?}", numbers);

    let _a = numbers[2];
    println!("{:?}", numbers);

    numbers

}


fn _strings(){
    let mut name = String::from("Nikki");
    name += " Singh";
    println!("{}", name);

}

fn _string_exp(){
    let borrowed: &str = "Hello"; 
    //"Hello" is a string literal stored in the compiled program.
    //borrowed is a reference to that text.
    //It doesn't own the text.


    let owned: String = borrowed.to_string();
    //This creates a new, owned string containing a copy of "Hello".

    let slice: &str = &owned;
    //This creates a reference to the text inside owned. It doesn't make another copy of the text.

    /*
    owned  ──────owns──────> "Hello" (heap buffer)
                               ▲
    slice  ──────borrows───────┘
    */

    println!("{borrowed}");
    println!("{owned}");
    println!("{slice}");

    println!("{}", owned.len());

}