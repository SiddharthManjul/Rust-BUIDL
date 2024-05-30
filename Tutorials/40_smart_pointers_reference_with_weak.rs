#[cfg(test)]

mod tests {

    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_pointers_with_weak() {

        #[derive(Debug, Clone)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>,
        }

        #[derive(Debug, Clone)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House>,
        }

        let house_1: Rc<House> = Rc::new(House {
            address_number: 123,
            street: "coding avenue".to_string(),
            furniture: RefCell::new(vec!())
        });

        let table: Rc<Furniture> = Rc::new(Furniture {
            id: "table1".to_string(),
            description: "kitchen table".to_string(),
            house: Rc::downgrade(&house_1)
        });

        let desk: Rc<Furniture> = Rc::new(Furniture {
            id: "desk1".to_string(),
            description: "office desk".to_string(),
            house: Rc::downgrade(&house_1)
        });

        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house_1);
    }
}