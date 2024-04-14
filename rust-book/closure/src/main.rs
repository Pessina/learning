use std::thread;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;

        for s in &self.shirts {
            match s {
                ShirtColor::Red => red += 1,
                &ShirtColor::Blue => blue += 1,
            }
        }

        if red > blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway = store.giveaway(user_pref1);

    println!("{:?}", giveaway);

    let user_pref2 = None;
    let giveaway = store.giveaway(user_pref2);

    println!("{:?}", giveaway);

    let add_one_v4 = |x| x + 1;
    let x = add_one_v4(1);

    let example_closure = |x| x;
    let x = example_closure(String::from("3"));

    let only_borrow = || println!("{x}");

    println!("{x}");
    only_borrow();
    println!("{x}");

    let mut list = String::new();
    let mut borrow_mutate = || list.push('3');

    // println!("{list}");
    borrow_mutate();
    println!("{list}");

    let list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // println!("Before defining closure: {:?}", list);

    let mut rect_list = [
        Rectangle {
            width: 8,
            height: 8,
        },
        Rectangle {
            width: 12,
            height: 64,
        },
        Rectangle {
            width: 15,
            height: 32,
        },
    ];

    let mut count_rect = 0;
    // let mut count_list = vec![];
    // let my_str = String::from("count");

    rect_list.sort_by_key(|r| {
        // count_list.push(my_str);
        count_rect += 1;
        r.width
    });
}
