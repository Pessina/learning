#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        dbg!("{}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alaska,
    NewYork,
    California,
    Colorado,
    Texas,
    Florida,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            dbg!("{}", state);
            25
        }
    }
}

fn add_one(value: Option<u32>) -> Option<u32> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let msg_quit = Message::Quit;
    let msg_move = Message::Move { x: 3, y: 2 };
    let msg_write = Message::Write(String::from("My name is Felipe"));
    let msg_change_color = Message::ChangeColor(255, 0, 10);

    let some_char = Some('a');
    let some_number = Some(32);
    let some_other_number = Some(18);
    let some_none: Option<u32> = None;

    if let (Some(some_number), Some(some_other_number)) = (some_number, some_other_number) {
        let t = some_number + some_other_number;
        println!("{t}");
    }

    msg_move.call();

    let x = Some(5);
    let x_one = add_one(x);
    let none = add_one(None);
}

fn dice_roll_use_catch_all(dice_value: u32) -> () {
    match dice_value {
        3 => println!("re-roll"),
        7 => println!("skip your turn"),
        other => println!("{}", other),
    }
}
fn dice_roll_dont_use_catch_all(dice_value: u32) -> u32 {
    match dice_value {
        3 => 3,
        7 => 7,
        _ => 0,
    }
}
fn dice_roll_dont_use_catch_all_dont_do_nothing(dice_value: u32) {
    match dice_value {
        3 => println!("re-roll"),
        7 => println!("skip your turn"),
        _ => (),
    }
}
