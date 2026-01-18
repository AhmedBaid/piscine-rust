use std::{cell::RefCell, rc::Rc};

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: usize,
    pub max: usize,
}
impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
            value: 0,
            max,
        }
    }
    pub fn set_value<T>(&self, value: &Rc<T>) {
        if Rc::strong_count(value) > self.max {
            self.messages
                .borrow_mut()
                .push(format!("Error: You can't go over your quota!",));
        } else if Rc::strong_count(value) * 100 / self.max > 70 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                Rc::strong_count(value) * 100 / self.max
            ));
        }
    }
    pub fn peek<T>(&self, value: &Rc<T>) {
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            Rc::strong_count(value) * 100 / self.max
        ));
    }
}
