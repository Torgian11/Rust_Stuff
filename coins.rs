#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin:Coin = Coin::Quarter(UsState::Alaska);
    // println!("{}", value_in_cents(coin));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{five:?}");
    println!("{six:?}");
    println!("{none:?}");

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("The state quarter is from {state:?}!");
    } else {
        count += 1;
        println!("New count is {count}");
    }
}
