fn main() {
    println!("-- Not mutable --------------------");
    let x = 5;
    println!("The address of x is {:p}", &x);
    let x = x + 1;
    println!("The address of x is {:p}", &x);

    {
        let x = x * 2;
        println!("The address of x is {:p}", &x);
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");
    println!("The address of x is {:p}", &x);

    println!("-- Mutable --------------------");
    let mut y = 5;
    println!("The address of y is {:p}", &y);

    y += 1;
    println!("The address of y is {:p}", &y);

    {
        y *= 2;
        println!("The address of y is {:p}", &y);
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The address of y is {:p}", &y);
    println!("The value of y in the outer scope is: {y}");

    println!("-- Mutable but shadowed ------------");

    let y = y + 1;
    println!("The address of shadowed y is {:p}", &y);
    println!("The value of y in the outer scope after shadowing is: {y}");
}
