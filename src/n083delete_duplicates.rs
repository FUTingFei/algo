pub use leetcode_prelude::ListNode;
pub use leetcode_prelude::linkedlist;
pub use leetcode_prelude;

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut curr_node = head.as_mut();
        
        while let Some(node) = curr_node {
            if let Some(next_node) = node.next.as_mut() {
                if next_node.val == node.val {
                    node.next = next_node.next.take();
                    curr_node = Some(node);
                } else {
                    curr_node = node.next.as_mut();
                }
            } else {
                break;
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test083() {
        let list = linkedlist![1,1,1,2];
        let res = linkedlist![1,2];
        assert_eq!(res, Solution::delete_duplicates(list));
    }

    #[test]
    fn test2() {
        let list = linkedlist![1,1,2,3,3];
        let res = linkedlist![1,2,3];
        assert_eq!(res, Solution::delete_duplicates(list));
    }
}