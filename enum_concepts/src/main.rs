//define enum
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    //use enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    //collect info by using struct...
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //or, enrich enum
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    
    //data associated with enum can be different tuple types
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr2::V4(127, 0, 0, 1);

    let loopback = IpAddr2::V6(String::from("::1"));
    
    //Option<T> is a special enum that does null handling
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
}

//operate on enum
fn route(ip_kind: IpAddrKind) {}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

//match construct
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


