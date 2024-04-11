pub struct Guess {
    guess: i32,
}

impl Guess {
    pub fn new(guess: i32) -> Guess {
        if guess > 1 && guess < 100 {
            Guess { guess }
        } else {
            panic!("Invalid guess")
        }
    }
}
pub struct Rectangle {
    x: i32,
    y: i32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.x > other.x && self.y > other.y
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Greetings from {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { x: 10, y: 10 };

        let smaller = Rectangle { x: 5, y: 5 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle { x: 10, y: 10 };

        let smaller = Rectangle { x: 5, y: 5 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_add() {
        assert_eq!(add(5, 2), 7);
    }

    #[test]
    fn it_greeting() {
        let greet_felipe = greeting("Felipe");
        assert!(
            greet_felipe.contains("Felipe"),
            "Greeting did not contain name, value was {}",
            greet_felipe
        )
    }

    #[test]
    #[should_panic(expected = "Invalid guess")]
    fn is_valid_guess() {
        Guess::new(3000);
    }

    #[test]
    fn it_work() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Invalid sum"))
        }
    }
}
