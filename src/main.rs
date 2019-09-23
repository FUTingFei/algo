use std::collections::LinkedList;
use algo::list_algo;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(4);
    list.push_back(5);
    list.push_back(1);
    list.push_back(9);
    list_algo::delete_node(&mut list, 5);
    println!("{:?}", list);
}