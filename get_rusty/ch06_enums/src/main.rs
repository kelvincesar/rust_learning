enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddrKind {
    fn some_function (){
        println!("I'm at the IpAddrKind fn");
    }
}
struct IpAddr {
    kind: IpAddrKind,
    address: String
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    let localhost = IpAddr {
        kind: IpAddrKind::V4(192, 168, 1, 128),
        address: String::from("192.168.1.128")
    };

    let some_number = Some(5);
    let some_str = Some("Hello");
    let nothing:Option<i32> = None;

    // Sum
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // or None;

    // error:
    //let sum = x + y;
    let sum = x + y.unwrap_or( 0);

    println!("Hello, world!");

    value_in_cents(Coin::Quarter(UsState::California));

    // Options
    let five = Some(5);
    let six = plus_one(five); // Returns Option<6>
    let none = plus_one(None); // Returns None

    if let Some(5) = five {
        println!("It's five");
    }
}

fn route(ip_kind: IpAddrKind) {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    //...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        //None => None,
        _ => None,
        Some(i) => Some(i+1)
        
    }
}
