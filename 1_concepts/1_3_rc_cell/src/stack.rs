use std::cell::RefCell;
use std::rc::Rc;

struct GlobalStack<T> {
    vec: Rc<RefCell<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    fn new() -> Self {
        GlobalStack {
            vec: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn push(&self, t: T) {
        self.vec.borrow_mut().push(t);
    }

    fn pop(&self) -> Option<T> {
        self.vec.borrow_mut().pop()
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        GlobalStack {
            vec: self.vec.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::GlobalStack;

    #[test]
    fn test_global_stack() {
        let stack = GlobalStack::<u32>::new();

        stack.push(1);
        stack.push(2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));

        let clone = stack.clone();

        clone.push(3);

        assert_eq!(stack.pop(), Some(3));
    }
}