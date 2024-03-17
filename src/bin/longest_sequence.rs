/**

    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None

**/

pub fn longest_sequence(s: &str) -> Option<&str> {
    let b = s.as_bytes();

    let mut start = 0;
    let mut stop = start;
    let mut longest = 0;
    let mut biggest_start = start;
    let mut biggest_stop = stop;
    while stop < s.len() {
        println!("{:?} =?= {:?}", b[stop], b[start]);
        if b[stop] == b[start] {
            stop = stop + 1;
        } else {
            if stop - start > longest {
                longest = stop - start;
                biggest_start = start;
                biggest_stop = stop;
            }
            start = stop;
            stop = start;
        }
    }
    if biggest_start == biggest_stop {
        return None;
    }
    return s.get(biggest_start..biggest_stop);
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::longest_sequence;

    #[test]
    fn test_longest_sequence_empty_string() {
        assert_eq!(longest_sequence(""), None)
    }
    #[test]
    fn test_longest_sequence_numbers() {
        assert_eq!(longest_sequence("0123456").unwrap(), "0");
    }
    #[test]
    fn test_longest_sequence_first() {
        assert_eq!(longest_sequence("aaabbb").unwrap(), "aaa");
    }
    #[test]
    fn test_longest_sequence_another() {
        assert_eq!(longest_sequence("ababbba").unwrap(), "bbb");
    }
}
