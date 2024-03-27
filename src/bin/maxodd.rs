struct Solution {}

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let zeros = s.chars().filter(|&c| c == '0').count();
        let ones = s.len() - zeros;

        let mut result = String::with_capacity(zeros + ones + 1);
        result.push_str(&"1".repeat(ones - 1));
        result.push_str(&"0".repeat(zeros));
        result.push('1');

        result
    }
    pub fn maximum_odd_binary_number_bad(mut s: String) -> String {
        let mut left = 0;
        let length = s.len();
        let mut right = length - 1;
        let mut last_one = 0;

        while left < right {
            if s.as_bytes()[left] < s.as_bytes()[right] {
                unsafe {
                    s.as_bytes_mut().swap(left, right);
                }
                last_one = left;
                left += 1;
            } else if s.as_bytes()[left] == b'1' {
                last_one = left;
                left += 1;
            } else {
                right -= 1;
            }
        }
        unsafe {
            s.as_bytes_mut()[last_one] = s.as_bytes()[length - 1];
            s.as_bytes_mut()[length - 1] = b'1';
        }
        s
    }
}

fn main() {
    assert_eq!(
        Solution::maximum_odd_binary_number(String::from("1010")),
        Solution::maximum_odd_binary_number_bad(String::from("1010"))
    );
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::maximum_odd_binary_number(String::from("010")),
            "001"
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::maximum_odd_binary_number(String::from("0101")),
            "1001"
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::maximum_odd_binary_number(String::from("1")), "1");
    }
}
