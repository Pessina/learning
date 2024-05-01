use oop::average_collection::base::AverageCollection;
use oop::draw::base::{Button, Input, Screen};
use oop::post::base::Post;

fn main() {
    let mut average_collection = AverageCollection::new();

    average_collection.add(1);
    average_collection.add(2);
    average_collection.add(3);
    average_collection.add(4);

    println!("{}", average_collection.average());

    average_collection.remove();
    println!("{}", average_collection.average());

    let mut screen = Screen::new();

    screen.add(Box::new(Button {
        width: 32,
        height: 32,
    }));

    screen.add(Box::new(Input {
        label: String::from("Title"),
        length: 32,
    }));

    screen.run();

    let mut post = Post::new();

    post.add_text("Hello world!");
    assert_eq!("", post.text());

    post.request_review();
    assert_eq!("", post.text());

    post.approve();
    assert_eq!("Hello world!", post.text());
}
