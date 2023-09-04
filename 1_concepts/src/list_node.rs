use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<ListNode<T>>>>,
    pub prev: Option<Weak<RefCell<ListNode<T>>>>,
}


impl<T> ListNode<T> {
    pub fn new(value: T) -> Self {
        ListNode {
            value,
            next: None,
            prev: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_node() {
        let node = ListNode::new(1);
        assert_eq!(node.value, 1);
        assert!(node.next.is_none());
        assert!(node.prev.is_none());
    }
}