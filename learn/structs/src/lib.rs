#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width >= other.width
    }
}


#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn test_larger_can_hold_smaller() {
        let larger_rectangle = Rectangle { width: 7, length: 7 };
        let smaller_rectangle = Rectangle { width: 5, length: 5 };

        assert!(larger_rectangle.can_hold(&smaller_rectangle));
        assert!(!smaller_rectangle.can_hold(&larger_rectangle));
    }

    #[test]
    fn test_smaller_cannot_hold_larger() {
        let larger_rectangle = Rectangle { width: 7, length: 7 };
        let smaller_rectangle = Rectangle { width: 5, length: 5 };

        assert!(!smaller_rectangle.can_hold(&larger_rectangle));
    }
}
