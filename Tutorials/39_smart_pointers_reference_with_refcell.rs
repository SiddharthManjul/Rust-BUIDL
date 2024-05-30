#[cfg(test)]

mod tests {

    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_pointers() {

        let x:Rc<RefCell<i32>>  = Rc::new(RefCell::new(50));
        let y:Rc<RefCell<i32>>  = Rc::clone(&x);

        *x.borrow_mut() = 72;

        dbg!(x.borrow());
        dbg!(y.borrow());

    }
}