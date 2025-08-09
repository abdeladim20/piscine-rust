use std::cell::RefCell;
use std::cell::Cell;
#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::<bool>::new()),
        }
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        // todo!()
        let id = self.states.borrow().len();
        self.states.borrow_mut().push(false);
        (id ,Thread::new(id, c, self))
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
        // todo!()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        // todo!()
        self.states.borrow()[id]
    }

    pub fn drop_thread(&self, id: usize) {
        // todo!()
        if self.is_dropped(id) {
            panic!("{} is already dropped", id);
        }
        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() +1);
    }
}

#[derive(Debug)]
pub struct Thread <'t> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'t ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Self {
            pid: p,
            cmd: c,
            parent: t, 
        }
    }

    pub fn skill(self) {
        // todo!()
        self.parent.drop_thread(self.pid);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {}
}