use std::sync::{Arc, Mutex};
use crate::list::List;
use crate::parallel_node::ParallelNode;

type Node<T> = Arc<Mutex<ParallelNode<T>>>;

fn new_node<T>(value: T) -> Option<Node<T>> {
    Some(Arc::new(Mutex::new(ParallelNode::new(value))))
}


pub struct ParallelDoublyLinkedList<T> {
    head: Arc<Mutex<Option<Node<T>>>>,
    tail: Arc<Mutex<Option<Node<T>>>>,
    length: Arc<Mutex<usize>>
}

impl<T> ParallelDoublyLinkedList<T> {
    pub fn new() -> Self {
        ParallelDoublyLinkedList {
            head: Arc::new(Mutex::new(None)),
            tail: Arc::new(Mutex::new(None)),
            length: Arc::new(Mutex::new(0)),
        }
    }

    fn new_head(&mut self, value: T) {
        let node = new_node(value);

        let mut length = self.length.lock().unwrap();
        let mut head = self.head.lock().unwrap();
        let mut tail = self.tail.lock().unwrap();

        *head = node.clone();
        *tail = node;

        *length = 1;
    }

    /// Function that appends node_right to node_left
    fn append_to_node_right(node_left_link: Node<T>, node_right_link: Node<T>) {
            let mut node_left = node_left_link.lock().unwrap();
            let mut node_right = node_right_link.lock().unwrap();

            node_right.next = None;
            node_right.prev = Some(Arc::downgrade(&node_left_link));

            node_left.next = Some(node_right_link.clone());
    }

    // only pop from tail, so next to left is tail
    fn pop_right(node_left_link: Node<T>) {
        let mut node_left = node_left_link.lock().unwrap();
        let mut node_right = node_left.next.take().unwrap();

        let mut node_right = node_right.lock().unwrap();

        node_right.prev = None;
    }

    fn get_node(&self, index: usize) -> Option<Node<T>> {
        let length = self.length.lock().unwrap();
        // need to lock tail to items not be removed while we are getting
        let tail = self.tail.lock().unwrap();

        if index >= *length {
            return None;
        }


        let head = self.head.clone();
        // need to lock head to items not be removed while we are getting
        let head = head.lock().unwrap();
        let mut node = head.clone().unwrap();

        for _ in 0..index {
            let next = node.lock().unwrap().next.clone().unwrap();
            node = next;
        }

        Some(node)
    }
}

impl<T> Clone for ParallelDoublyLinkedList<T> {
    fn clone(&self) -> Self {
        ParallelDoublyLinkedList {
            head: self.head.clone(),
            tail: self.tail.clone(),
            length: self.length.clone(),
        }
    }
}

impl<T> List<T> for ParallelDoublyLinkedList<T>
where T: Copy
{
    fn push(&mut self, value: T) -> Option<usize> {
        return if self.head.lock().unwrap().is_none() {
            self.new_head(value);
            Some(0)
        } else {
            let new_node = new_node(value).unwrap();
            let mut length = self.length.lock().unwrap();
            let mut tail_option = self.tail.lock().unwrap();

            let tail = tail_option.as_ref().unwrap();


            let index = *length;
            ParallelDoublyLinkedList::append_to_node_right(tail.clone(), new_node.clone());

            *tail_option = Some(new_node);
            *length += 1;

            Some(index)
        }
    }

    fn pop(&mut self) -> Option<T> {
        let mut length = self.length.lock().unwrap();
        let mut tail_option = self.tail.lock().unwrap();

        return if *length == 0 {
            None
        } else if *length == 1 {
            let tail = tail_option.take().unwrap();
            let value = tail.lock().unwrap().value;

            self.head.lock().unwrap().take();

            *length = 0;

            Some(value)
        } else {
            let tail = tail_option.take().unwrap();
            let value = tail.lock().unwrap().value;
            let prev = tail.lock().unwrap().prev.clone().unwrap().upgrade().unwrap();

            ParallelDoublyLinkedList::pop_right(prev.clone());

            *tail_option = Some(prev);
            *length -= 1;

            Some(value)
        }
    }

    fn get(&self, index: usize) -> Option<T> {
        let node = self.get_node(index);
        node.map(|n| n.lock().unwrap().value)
    }

    fn update(&mut self, index: usize, value: T) -> Option<T> {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use crate::list::List;
    use crate::real_parallel_list::ParallelDoublyLinkedList;

    #[test]
    fn test_parallel_doubly_linked_list() {
        let list = ParallelDoublyLinkedList::<u32>::new();

        assert_eq!(list.length.lock().unwrap().clone(), 0);
        assert!(list.head.lock().unwrap().clone().is_none());
        assert!(list.tail.lock().unwrap().clone().is_none());
    }

    #[test]
    fn test_parallel_doubly_linked_list_push() {
        let mut list = ParallelDoublyLinkedList::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.length.lock().unwrap().clone(), 3);
        assert_eq!(list.head.lock().unwrap().clone().unwrap().lock().unwrap().value, 1);
        assert_eq!(list.tail.lock().unwrap().clone().unwrap().lock().unwrap().value, 3);
    }

    #[test]
    fn test_parallel_doubly_linked_list_push_parallel() {
        let mut list = ParallelDoublyLinkedList::<u32>::new();

        let mut list_clone = list.clone();

        let handle = std::thread::spawn(move || {
            list_clone.push(1);
            list_clone.push(2);
            list_clone.push(3);
        });

        let pos4= list.push(4);
        let pos5 = list.push(5);
        let pos6 = list.push(6);

        handle.join().unwrap();

        assert_eq!(list.length.lock().unwrap().clone(), 6);

        assert!(pos4.is_some());
        assert!(pos5.is_some());
        assert!(pos6.is_some());

        assert_eq!(list.get(pos4.unwrap()).unwrap(), 4);
        assert_eq!(list.get(pos5.unwrap()).unwrap(), 5);
        assert_eq!(list.get(pos6.unwrap()).unwrap(), 6);
    }

    #[test]
    fn test_parallel_doubly_linked_list_pop() {
        let mut list = ParallelDoublyLinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_parallel_doubly_linked_list_pop_parallel() {
        let mut list = ParallelDoublyLinkedList::<u32>::new();

        let mut list_clone = list.clone();

        let handle = std::thread::spawn(move || {
            list_clone.push(1);
            list_clone.push(2);
            list_clone.push(3);
        });

        handle.join().unwrap();

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn test_parallel_doubly_linked_list_many() {
        let list = ParallelDoublyLinkedList::<u32>::new();

        let mut handles = vec![];

        for i in 0..10 {
            let mut list_clone = list.clone();
            let handle = std::thread::spawn(move || {
                println!("Pushed in thread {} on pos {:?}", i, list_clone.push(i));
                println!("Pushed in thread {} on pos {:?}", i, list_clone.push(i + 1));

                println!("Popped in thread {} a value {:?}", i, list_clone.pop());

                println!("Pushed in thread {} on pos {:?}", i, list_clone.push(i + 2));

                println!("Popped in thread {} a value {:?}", i, list_clone.pop());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(list.length.lock().unwrap().clone(), 10);
    }
}

