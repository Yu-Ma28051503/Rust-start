/* match制御フロー演算 */

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaba,
    Alaska,
}

fn value_in_conins(coin :Coin) -> u32 {
    let mut count = 0;

    match coin {
        Coin::Penny  => {
            println!("Luccy Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime   => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            25
        },
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("hello again!");
    }
    else {
        count += 1;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    println!("Hello, world!");
    // println!("Penny: {}", value_in_conins(Penny));

    let five = 5;
    let six  = plus_one(five);
    let none = plus_one(None);

    let some_u8 = 0u8;
    match some_u8 {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three").
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    
}
