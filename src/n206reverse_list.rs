use leetcode_prelude::ListNode;
use leetcode_prelude::linkedlist;
use leetcode_prelude;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut old_l = head;
    let mut new_l = None;

    while let Some(mut node) = old_l {
        old_l = node.next.take(); // 1. 原链表更新为出去原来头结点的新链表
        node.next = new_l;  // 2. 把节点拿下来接到新链表的前面
        new_l = Some(node); // 3. 生成新链表
    }

    new_l
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let list1 = linkedlist![1,2,3,4,5];
        let res = linkedlist![5,4,3,2,1];
        assert_eq!(res, reverse_list(list1));
    }

}