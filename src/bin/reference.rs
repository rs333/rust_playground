use std::cell::RefCell;
use std::rc::Rc;

fn c<F: FnOnce() + 'static>(f: F) {
    f();
}

fn test(v: Rc<RefCell<Vec<i32>>>, val: i32) {
    v.borrow_mut().push(val);
}

fn test2(v: &mut Vec<i32>, val: i32) {
    v.push(val);
}

fn main() {
    let v = Rc::new(RefCell::new(vec![1, 2, 3]));
    let mut z = vec![1, 2];
    test(v.clone(), 42);
    test2(&mut z, 42);
    println!("{:?}", z);
    test2(&mut z, 42);
    println!("{:?}", z);

    c({
        let v = v.clone();
        move || {
            println!("inner 1: {:?}", v);
            v.borrow_mut().push(4);
        }
    });
    c({
        let v = v.clone();
        move || {
            println!("inner 1: {:?}", v);
            v.borrow_mut().push(5);
        }
    });

    println!("outer: {:?}", v);
    v.borrow().iter().for_each(|x| println!("{x}"));
    println!("outer: {:?}", v);
}
