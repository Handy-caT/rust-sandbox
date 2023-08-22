use std::cell::RefCell;
use std::rc::Rc;
use crate::list::List;
use crate::list_node::ListNode;

type Node<T> = Rc<RefCell<ListNode<T>>>;

fn new_node<T>(value: T) -> Node<T> {
    Rc::new(RefCell::new(ListNode::new(value)))
}

pub struct DoublyLinkedList<T> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn new_head(&mut self, value: T) {
        let node = new_node(value);

        self.head = Some(node.clone());
        self.tail = Some(node);

        self.length = 1;
    }

    /// Function that appends node_right to node_left
    fn append_to_node_right(node_left: Node<T>, node_right: Node<T>) {
        if node_left.borrow().next.is_none() {
            // if no next node is tail, no need to get next one
            node_right.borrow_mut().next = None;
            node_right.borrow_mut().prev = Some(Rc::downgrade(&node_left));

            node_left.borrow_mut().next = Some(node_right);
        } else {
            // we are not appending to the tail, so need to link with
            // next to left node
            let next = node_left.borrow_mut().next.take().unwrap();
            node_left.borrow_mut().next = Some(node_right.clone());

            next.borrow_mut().prev = Some(Rc::downgrade(&node_right));

            node_right.borrow_mut().next = Some(next);
            node_right.borrow_mut().prev = Some(Rc::downgrade(&node_left));
        }
    }

    fn pop_node_right(node_left: Node<T>) {
        let node_right = node_left.borrow_mut().next.take().unwrap();

        if node_right.borrow().next.is_none() {
            // right node is tail, just set node_left next to None
            node_left.borrow_mut().next = None;

            node_right.borrow_mut().prev.take();
        } else {
            // right node is not tail, so need to link left node
            // with right node's next
            let next = node_right.borrow_mut().next.take().unwrap();
            node_left.borrow_mut().next = Some(next.clone());

            next.borrow_mut().prev = Some(Rc::downgrade(&node_left));
        }
    }

    fn pop_node_left(node_right: Node<T>) {
        let node_left = node_right.borrow_mut().prev.take().unwrap().upgrade().unwrap();

        if node_left.borrow().prev.is_none() {
            // left node is head, just set node_right prev to None
            node_right.borrow_mut().prev = None;

            node_left.borrow_mut().next.take();
        } else {
            // left node is not head, so need to link right node
            // with left node's prev
            let prev = node_left.borrow_mut().prev.take().unwrap().upgrade().unwrap();
            node_right.borrow_mut().prev = Some(Rc::downgrade(&prev));

            prev.borrow_mut().next = Some(node_right);
        }
    }



    fn get_node(&self, index: usize) -> Option<Node<T>> {
        if index >= self.length {
            return None;
        }

        let mut node = self.head.clone();

        for _ in 0..index {
            node = node.unwrap().borrow().next.clone();
        }

        node
    }
}

impl<T> List<T> for DoublyLinkedList<T>
where T: Copy
{
    fn push(&mut self, value: T) -> Option<usize> {
        // if no head, create head
        if self.head.is_none() {
            self.new_head(value);
            Some(0)
        } else {
            // if head is present
            let node = new_node(value);
            let tail = self.tail.take().unwrap();
            DoublyLinkedList::append_to_node_right(tail, node.clone());

            self.tail = Some(node);
            self.length += 1;

            Some(self.length - 1)
        }
    }

    fn pop(&mut self) -> Option<T> {
        return if self.length == 0 {
            None
        } else if self.length == 1 {
            self.tail.take();
            let node = self.head.take();
            self.length = 0;

            node.map(|n| n.borrow_mut().value)
        } else {
            let tail = self.tail.take().unwrap();
            let value = tail.borrow_mut().value;
            let prev = tail.borrow().prev.clone().unwrap().upgrade().unwrap();

            DoublyLinkedList::pop_node_right(prev.clone());

            self.tail = Some(prev);
            self.length -= 1;

            Some(value)
        }
    }

    fn get(&self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        let node = self.get_node(index).unwrap();
        let value = node.borrow().value;
        Some(value)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        if index == self.length - 1 {
            return self.pop();
        } else if index == 0 {
            let head = self.head.take().unwrap();
            let value = head.borrow_mut().value;
            let next = head.borrow().next.clone().unwrap();

            DoublyLinkedList::pop_node_left(next.clone());

            self.head = Some(next);
            self.length -= 1;

            return Some(value);
        }

        let node = self.get_node(index).unwrap();
        let value = node.borrow().value;
        let prev = node.borrow().prev.clone().unwrap().upgrade().unwrap();

        DoublyLinkedList::pop_node_right(prev.clone());

        self.length -= 1;
        Some(value)
    }

    fn update(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.length {
            return None;
        }

        let node = self.get_node(index).unwrap();
        let old_value = node.borrow_mut().value;
        node.borrow_mut().value = value;

        Some(old_value)
    }

    fn clear(&mut self) {
        self.head.take();
        self.tail.take();
        self.length = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::doubly_linked_list::DoublyLinkedList;
    use crate::list::List;

    #[test]
    fn test_doubly_linked_list_new() {
        let list = DoublyLinkedList::<u32>::new();
        assert_eq!(list.length, 0);
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_doubly_linked_list_new_head() {
        let mut list = DoublyLinkedList::<u32>::new();

        list.new_head(1);

        assert_eq!(list.length, 1);
        assert!(list.head.is_some());
        assert!(list.tail.is_some());
    }

    #[test]
    fn test_doubly_linked_list_push() {
        let mut list = DoublyLinkedList::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.length, 3);
        assert!(list.head.is_some());
        assert!(list.tail.is_some());
    }

    #[test]
    fn test_doubly_linked_list_pop_head() {
        let mut list = DoublyLinkedList::<u32>::new();

        assert_eq!(list.pop(), None);

        list.push(1);

        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn test_doubly_linked_list_pop_tail() {
        let mut list = DoublyLinkedList::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_doubly_linked_list_get() {
        let mut list = DoublyLinkedList::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn test_doubly_linked_list_update_head() {
        let mut list = DoublyLinkedList::<u32>::new();

        list.push(1);

        assert_eq!(list.update(0, 2), Some(1));
        assert_eq!(list.update(0, 3), Some(2));

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.update(0, 2), None);

        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_doubly_linked_list_clear() {
        let mut list = DoublyLinkedList::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        list.clear();

        assert_eq!(list.length, 0);
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_doubly_linked_list_remove() {
        let mut list = DoublyLinkedList::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.remove(1), Some(2));
        assert_eq!(list.remove(1), Some(3));
        assert_eq!(list.remove(1), None);
        assert_eq!(list.remove(0), Some(1));

        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_doubly_linked_list_remove_head() {
        let mut list = DoublyLinkedList::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.remove(0), Some(1));
        assert_eq!(list.remove(0), Some(2));
        assert_eq!(list.remove(0), Some(3));

        assert_eq!(list.remove(0), None);
    }

}
