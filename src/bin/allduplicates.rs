use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut result: HashSet<i32> = HashSet::new();

        for val in nums {
            if !seen.insert(val) {
                result.insert(val);
            }
        }

        result.into_iter().collect()
    }
}
fn main() {
    assert_eq!(Solution::find_duplicates(vec![4, 2, 3]).len(), 0);
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        let test_cases = vec![
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]),
            (vec![1, 1, 2, 1], vec![1]),
            (vec![1], vec![]),
        ];
        for (input, output) in test_cases {
            let result: Vec<i32> = Solution::find_duplicates(input);
            assert_eq!(
                result.len(),
                output.len(),
                "Result length did not match expected"
            );
        }
    }
}
