use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

// if your are reading this im really sorry
// this is the worst unreadable unoptimized garbage code ive ever made
// dear god dont even try
// some times rust is just a pain in the ass

#[macro_export]
macro_rules! use_state {
    ($t:expr) => {
        // use hooks::use_state::use_state;
        use_state(concat!(file!(), ":", line!(), ":", column!()), $t)
    };
}

thread_local! {
    pub static USE_STATE_MAP: RefCell<HashMap<&'static str, Rc<dyn Any>>> =
        RefCell::new(HashMap::new());
}

pub fn use_state<T: Display + Clone + 'static>(id: &'static str, t: T) -> StateBox<T> {
    USE_STATE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        let entry = map
            .entry(id)
            .or_insert_with(|| Rc::new(RefCell::new(t)) as Rc<dyn Any>);

        let state_ref = Rc::downcast::<RefCell<T>>(entry.clone()).unwrap();
        StateBox::new(state_ref)
    })
}

pub struct StateBox<T> {
    pub state_ref: Rc<RefCell<T>>,
}

impl<T: Clone> StateBox<T> {
    pub fn new(state_ref: Rc<RefCell<T>>) -> Self {
        Self { state_ref }
    }
    pub fn get(&self) -> T {
        self.state_ref.borrow().clone()
    }
    pub fn set(&self, t: T) {
        *self.state_ref.borrow_mut() = t;
    }
}
