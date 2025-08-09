use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Tracker {
        Self {
            messages: RefCell::new(Vec::<String>::new()),
            value: RefCell::new(0),
            max: max,
        }
    }
    pub fn set_value(&self, rc: &Rc<u32>) {
        let count = Rc::strong_count(rc);
        if self.max < count {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else {
            *self.value.borrow_mut() = count;
            let percent = (count * 100) / self.max;
            if percent > 70 {
                self.messages.borrow_mut().push(format!(
                    "Warning: You have used up over {percent}% of your quota!"
                ));
            }
        }
    }
    pub fn peek(&self, rc: &Rc<u32>) {
        let count = Rc::strong_count(rc);
        let percent = (count * 100) / self.max;
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {percent}% of your quota"
        ));
    }
}
