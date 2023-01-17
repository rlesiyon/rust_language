enum IpAddrKind {
    V4(u8, u8, u8, u8), 
    V6(String),
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:#?}", self)
    }
}

fn main() {
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    //println!("{:#?}, {:#?}", four, six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}\n, {:?}", home, loopback);
    
    // enum with implementation same as struct. 
    let m = Message::Write(String::from("hello"));
    m.call();

    let message_change_color = Message::ChangeColor((55), (100), (250));
    message_change_color.call();

    // options
    let x = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}, {:?}", x, some_char, absent_number);

    // control flow with match: 
    let coin = Coin::Quarter(UsState::Alabama);

    println!("The cents in {:?}: {}", coin, value_in_cents(&coin));

    //concise control with let.
    // matches one pattern while ignoring the other.
    //
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // if let method
    if let Some(max) = config_max {
        println!("\nThe maximum is configured to be {}", max)
    }

    // penny only.

    if let Coin::Quarter(state) = coin {
        println!("The coin was made in {:?} state", state);
    }

}


// match control
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        },
    }
}

 //concise control with let.