use super::pointer::*;

pub struct Node {
    parent: WeakPointer,
    children: Vec<Pointer>,
}
