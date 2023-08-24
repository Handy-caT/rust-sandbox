use crate::doubly_linked_list::DoublyLinkedList;
use crate::list::List;
use crate::real_parallel_list::ParallelDoublyLinkedList;

mod doubly_linked_list;
mod list_node;
mod list;
mod parallel_list;
mod parallel_node;
mod real_parallel_list;

fn main() {
    let mut list = ParallelDoublyLinkedList::<u32>::new();

    for i in 1..=1000 {
        list.push(i);
    }

    list.clear();
}
