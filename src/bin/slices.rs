fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

#[allow(dead_code)]
fn word(s: &str, word: usize) -> &str {
    let bytes = s.as_bytes();

    let mut start: usize = 0;
    let mut count = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if word == 1 {
                return &s[start..i];
            }
            count += 1;
            if count == word - 1 {
                start = i + 1;
            } else if count == word {
                return &s[start..i];
            }
        }
    }

    &s[start..]
}

fn main() {
    let mut s = String::from("Hello World");

    let slice = first_word(&s);
    println!("First word of {s} is {slice}");
    s.clear();
}

#[cfg(test)]
mod slice_tests {
    use crate::{first_word, word};

    #[test]
    fn test_first_word() {
        let s: String = "My Chicken!".to_string();
        assert_eq!(first_word(&s), "My");
    }
    #[test]
    fn test_any_word() {
        let s: String = "My Chicken and Fries!".to_string();
        assert_eq!(word(&s, 1), "My");
        assert_eq!(word(&s, 2), "Chicken");
        assert_eq!(word(&s, 3), "and");
        assert_eq!(word(&s, 4), "Fries!");
    }
}
