use std::time::Instant;
pub fn reverse(mut x: i32) -> i32 {
    let mut result: i64 = 0;
    while x != 0 {
        result = result * 10 + (x % 10) as i64;
        x /= 10;
    }
    let i32result = result as i32;
    if i32result as i64 == result {
        i32result
    } else {
        0
    }
}
pub fn reverse2(mut x: i32) -> i32 {
    let mut result: i32 = 0;
    while x != 0 {
        result = if let Some(val) = result.checked_mul(10) {
            if let Some(y) = val.checked_add(x % 10) {
                y
            } else {
                return 0;
            }
        } else {
            return 0;
        };
        x /= 10;
    }
    result
}

pub fn reverse3_helper(mut x: i32) -> Option<i32> {
    let mut result: i32 = 0;
    while x != 0 {
        result = result.checked_mul(10)?;
        result = result.checked_add(x % 10)?;
        x /= 10;
    }
    Some(result)
}

pub fn reverse3(x: i32) -> i32 {
    reverse3_helper(x).unwrap_or_default()
}

fn main() {
    const COUNT: usize = 2_000_000_000;
    let start = Instant::now();
    for _ in 0..=COUNT {
        assert_eq!(reverse(1534236469), 0);
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
    }
    let duration = start.elapsed();
    eprintln!("Execution time {:?}", duration);

    let start = Instant::now();
    for _ in 0..=COUNT {
        assert_eq!(reverse2(1534236469), 0);
        assert_eq!(reverse2(123), 321);
        assert_eq!(reverse2(-123), -321);
    }
    let duration = start.elapsed();
    eprintln!("Execution time {:?}", duration);

    let start = Instant::now();
    for _ in 0..=COUNT {
        assert_eq!(reverse3(1534236469), 0);
        assert_eq!(reverse3(123), 321);
        assert_eq!(reverse3(-123), -321);
    }
    let duration = start.elapsed();
    eprintln!("Execution time {:?}", duration);
}
