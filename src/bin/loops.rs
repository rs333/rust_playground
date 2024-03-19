use std::collections::HashMap;

fn counter() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    result
}

fn labeled_loop() -> (i32, i32) {
    let mut count = 0;
    let mut remaining;

    'outer_loop: loop {
        remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    (count, remaining)
}

fn while_loop() -> i32 {
    let a = [10, 20, 30];
    let mut index = 0;
    let mut result = 0;
    while index < a.len() {
        result += a[index];
        index += 1;
    }
    result
}

fn main() {}

fn for_element_loop() -> i32 {
    let a = [10, 20, 30];

    let mut result = 0;
    for element in a {
        result += element;
    }
    result
}
fn for_ranged_loop() -> i32 {
    let mut result = 1;
    // While not obvious, this starts at 3
    // vice 4 and counts backwards.
    for element in (0..4).rev() {
        println!(" {element}");
        result *= element;
        if element == 2 {
            break;
        }
    }
    result
}

fn fib(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    if n < 2 {
        return 1;
    }
    if memo.contains_key(&n) {
        return *memo.get(&n).unwrap();
    }
    // Need a map for memoization
    let result = fib(n - 1, memo) + fib(n - 2, memo);
    memo.insert(n, result);
    return result;
}
#[cfg(test)]
mod tests {
    use crate::{counter, fib, for_element_loop, for_ranged_loop, labeled_loop, while_loop};

    #[test]
    fn test_loop() {
        assert_eq!(counter(), 20);
    }
    #[test]
    fn test_labeled_loop() {
        assert_eq!(labeled_loop(), (2, 10));
    }

    #[test]
    fn test_while() {
        assert_eq!(while_loop(), 60);
    }

    #[test]
    fn test_for_element_loop() {
        assert_eq!(for_element_loop(), 60);
    }

    #[test]
    fn test_for_ranged_loop() {
        assert_eq!(for_ranged_loop(), 6);
    }

    #[test]
    fn test_fib() {
        let mut memo: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        assert_eq!(fib(0, &mut memo), 1);
        assert_eq!(fib(1, &mut memo), 1);
        assert_eq!(fib(2, &mut memo), 2);
        assert_eq!(fib(3, &mut memo), 3);
        assert_eq!(fib(4, &mut memo), 5);
        assert_eq!(fib(40, &mut memo), 165580141);
    }
}
