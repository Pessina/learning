use rand;

pub fn add_one(x: u8) -> u8 {
    let y = rand::random::<u8>();
    x + y
}

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn it_add_one() {
        let x = add_one(3);
        assert!(x > 3)
    }
}
