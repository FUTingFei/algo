pub mod list_algo {
    use std::collections::LinkedList;
    pub fn delete_node(list: &mut LinkedList<u32>, val: u32) {
        println!("{:?},{}", list,val);
        // for l in list {
            // if *l == val {
            //     l.next() = l.next().next(); 好像rust linkedlist 没有next这个功能呢_(:зゝ∠)_，这可咋整
            //     break;
            // }
        // }
    }
}