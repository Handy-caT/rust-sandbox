use std::sync::Weak;
use std::sync::{Arc, Mutex};

pub struct ParallelNode<T> {
    pub value: T,
    pub next: Option<Arc<Mutex<ParallelNode<T>>>>,
    pub prev: Option<Weak<Mutex<ParallelNode<T>>>>
}

impl<T> ParallelNode<T> {
    pub fn new(value: T) -> Self {
        ParallelNode {
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
    fn test_parallel_node() {
        let node = ParallelNode::new(1);
        assert_eq!(node.value, 1);
        assert!(node.next.is_none());
        assert!(node.prev.is_none());
    }
}