#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
}

fn filter_shoes_size<'a>(shoes: &'a Vec<Shoe>, size: u32) -> Vec<&'a Shoe> {
    shoes.iter().filter(|s| s.size > size).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_filter_shoes_size() {
        let shoes = vec![
            Shoe { size: 32 },
            Shoe { size: 33 },
            Shoe { size: 35 },
            Shoe { size: 37 },
        ];

        let expected = vec![&shoes[2], &shoes[3]];

        assert_eq!(expected, filter_shoes_size(&shoes, 34))
    }
}
