fn main() {
    print_labeled_measurement(5, 'h');
    println!("five() = {}", five());
    println!("plus_one(5) = {}", plus_one(5));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(a: i32) -> i32 {
    a + 1
}

#[cfg(test)]
mod tests {
    use crate::{five, plus_one};

    #[test]
    fn test_five() {
        assert_eq!(five(), 5);
    }

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(42), 43);
    }
}
