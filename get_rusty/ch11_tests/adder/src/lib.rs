#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

pub fn add_two (a: i32) -> i32 {
    a + 2
}

pub fn greeting (name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new (value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("O valor deve ser entre 1 e 100, recebido: {}", value);
        }

        Guess { value }
    }
}
// Esse módulo só é compilado quando `cargo test` é executado
#[cfg(test)]
mod tests {
    use super::*;    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    #[ignore]
    fn failing_test() {
        panic!("Make this test fail");
    }
    

    
    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            w: 8,
            h: 7
        };

        let smaller = Rectangle {
            w: 5,
            h: 1
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_can_hold_smaller(){
        let larger = Rectangle {
            w: 8,
            h: 7
        };

        let smaller = Rectangle {
            w: 5,
            h: 1
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Kelvin");

        assert!(
            result.contains("Kelvin"),
            "Greeting não contém `name`, o valor retornado foi ´{}´",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
