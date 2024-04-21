fn main() {
    let mut s = String::from("Hello!");

    // This function borrows the string.
    borrows(&mut s);
    borrows(&mut s);
    borrows(&mut s);

    // This one takes ownership
    takes_ownership(s);

    // Uncommenting this will result
    // in an error because the prevous call
    // takes ownership of the string.
    // takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn borrows(some_string: &mut String) {
    some_string.push('B');
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
