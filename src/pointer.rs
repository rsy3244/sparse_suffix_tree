use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use super::node::Node;

pub struct Pointer(Rc<RefCell<Node>>);

impl Deref for Pointer {
    type Target = Rc<RefCell<Node>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct WeakPointer(Weak<RefCell<Node>>);
