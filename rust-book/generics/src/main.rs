struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &U {
        &self.y
    }
}

impl Point<i32, i32> {
    fn get_x_i32(&self) -> i32 {
        self.x
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        return Point {
            x: self.x,
            y: other.y,
        };
    }
}

fn main() {
    let mut vec = vec![5, 7, 2, 7, 8, 12, 68, 9, 234, 27, 8, 2];

    let a = get_largest(&vec);
    println!("{}", a);

    vec.push(3);

    let point = Point { x: 2, y: 3 };
    let point_2 = Point { x: 2, y: 3.0 };

    println!("{}", point.get_x());
    println!("{}", point.get_x_i32());
    println!("{}", point_2.get_y());

    let point_3 = point.mixup(point_2);

    print!("{}, {}", point_3.get_x(), point_3.get_y())
}

fn get_largest<T: std::cmp::PartialOrd>(vec: &Vec<T>) -> &T {
    let mut largest = &vec[0];

    for number in vec {
        if number > largest {
            largest = number;
        }
    }

    largest
}
