pub struct Guess {
    pub value: i32,
}

impl Guess {
    // Acts like a constructor.
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "The secret number is less than or equal to 100, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            );
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn grater_than_100() {
        Guess::new(200);
    }
}
