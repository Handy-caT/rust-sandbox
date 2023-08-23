use std::sync::Weak;
use std::sync::{Arc, Mutex};

pub struct ParallelNode<T> {
    pub value: T,
    pub next: Arc<Mutex<Option<ParallelNode<T>>>>,
    pub prev: Weak<Mutex<Option<ParallelNode<T>>>>
}

impl<T> ParallelNode<T> {
    pub fn new(value: T) -> Self {
        ParallelNode {
            value,
            next: Arc::new(Mutex::new(None)),
            prev: Arc::downgrade(&Arc::new(Mutex::new(None))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_node() {
        let node = ParallelNode::new(1);
        assert_eq!(node.value, 1);
        assert!(node.next.lock().unwrap().is_none());
        assert!(node.prev.upgrade().is_none());
    }
}