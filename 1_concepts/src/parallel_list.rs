use std::sync::{Arc, Mutex};
use crate::doubly_linked_list::DoublyLinkedList;
use crate::list::List;


pub struct ParallelDoublyLinkedList<T> {
    list: Arc<Mutex<DoublyLinkedList<T>>>
}

impl<T> ParallelDoublyLinkedList<T> {
    pub fn new() -> Self {
        ParallelDoublyLinkedList {
            list: Arc::new(Mutex::new(DoublyLinkedList::<T>::new()))
        }
    }
}


impl<T> Clone for ParallelDoublyLinkedList<T> {
    fn clone(&self) -> Self {
         ParallelDoublyLinkedList {
             list: self.list.clone()
         }
    }
}

unsafe impl<T> Send for ParallelDoublyLinkedList<T> {}
unsafe impl<T> Sync for ParallelDoublyLinkedList<T> {}

impl<T> List<T> for ParallelDoublyLinkedList<T>
where T: Copy
{
    fn push(&mut self, value: T) -> Option<usize> {
        let mut list = self.list.lock().unwrap();
        list.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        let mut list = self.list.lock().unwrap();
        list.pop()
    }

    fn get(&self, index: usize) -> Option<T> {
        let list = self.list.lock().unwrap();
        list.get(index)
    }

    // fn remove(&mut self, index: usize) -> Option<T> {
    //     let mut list = self.list.lock().unwrap();
    //     list.remove(index)
    // }

    fn update(&mut self, index: usize, value: T) -> Option<T> {
        let mut list = self.list.lock().unwrap();
        list.update(index, value)
    }

    fn clear(&mut self) {
        let mut list = self.list.lock().unwrap();
        list.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_doubly_linked_list() {
        let list = ParallelDoublyLinkedList::<u32>::new();

        assert_eq!(list.list.lock().unwrap().length(), 0);
    }

    #[test]
    fn test_parallel_doubly_linked_list_push() {
        let mut list = ParallelDoublyLinkedList::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.list.lock().unwrap().length(), 3);
        assert_eq!(list.list.lock().unwrap().get(0).unwrap(), 1);
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

        assert_eq!(list.list.lock().unwrap().length(), 6);

        assert!(pos4.is_some());
        assert!(pos5.is_some());
        assert!(pos6.is_some());

        assert_eq!(list.list.lock().unwrap().get(pos4.unwrap()).unwrap(), 4);
        assert_eq!(list.list.lock().unwrap().get(pos5.unwrap()).unwrap(), 5);
        assert_eq!(list.list.lock().unwrap().get(pos6.unwrap()).unwrap(), 6);
    }

}
