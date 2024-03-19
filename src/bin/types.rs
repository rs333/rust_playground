fn main() {
    println!("A simple rust program.");
    let mut a: i8 = 42;

    while a != 41 {
        a = a.wrapping_add(1);

        println!("a = {a}, a/2 = {},  a % 5 = {}", a / 2, a % 5);
    }
    a = a / 2;
    println!("a/2 = {a}");

    let tup: (i32, f64, u8, char) = (500, 6.4, 25, 'a');
    println!("Print a tuple? {},{},{},{}", tup.0, tup.1, tup.2, tup.3);

    let pairs = [(1, 'a'), (2, 's'), (3, 'd')];
    println!("Pairs: {} ", pairs[0].1);
}
