use crate::doubly_linked_list::DoublyLinkedList;
use crate::list::List;

mod doubly_linked_list;
mod list_node;
mod list;

fn main() {
    let mut list = DoublyLinkedList::<u32>::new();

    for i in 1..=1000 {
        list.push(i);
    }

    list.clear();
}
