use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Cell<T> {}
impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);
        let t1 = std::thread::spawn(move || {
            eprintln!("In t1: {}", x1.get());
            x1.set(1);
        });
        let x2 = Arc::clone(&x);
        let t2 = std::thread::spawn(move || {
            eprintln!("In t2: {}", x2.get());
            x2.set(2);
        });
        t1.join();
        t2.join();
    }
}
