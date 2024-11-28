fn median(data: &mut [i32]) -> Option<f32> {
    let half = match data.len() {
        0 => return None,
        1 => return Some(data.first().copied().unwrap_or_default() as f32),
        _ => data.len() / 2,
    };
    data.sort();
    let mut result = data.get(half).copied().unwrap() as f32;
    if data.len() % 2 == 0 {
        result += data.get(half - 1).copied().unwrap() as f32;
        result /= 2.0;
    }
    Some(result)
}
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    let result = median(&mut data);
    println!("Median: {}", result.unwrap());
    data.push(10);
    let result = median(&mut data);
    println!("Median: {}", result.unwrap());
    data.push(100);
    let result = median(&mut data);
    println!("Median: {}", result.unwrap());
}
