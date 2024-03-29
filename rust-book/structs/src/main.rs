struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width > target.width && self.height > target.height
    }
    fn square(dimension: u32) -> Self {
        Self {
            width: dimension,
            height: dimension,
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 32,
        width: 32,
    };
    let rect2 = Rectangle {
        height: 24,
        width: 24,
    };
    let rect3 = Rectangle {
        height: 24,
        width: 33,
    };

    let square = Rectangle::square(2);

    println!("{}", rect.area());
    println!("{}", rect.can_hold(&rect2));
    println!("{}", rect.can_hold(&rect3));
    println!("{}", square.area());
}
