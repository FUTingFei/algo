pub use leetcode_prelude::ListNode;
pub use leetcode_prelude::linkedlist;
pub use leetcode_prelude;

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut l = ListNode::new(0);
        l.next = head;
        let mut l = Some(Box::new(l));

        let mut curr = l.as_mut();

        while let Some(node) = curr {
            if let Some(d) = node.next.as_mut() {
                if d.val == val {
                    let next = d.next.take();
                    node.next = next;
                    curr = Some(node);
                } else {
                    curr = node.next.as_mut();
                }
            } else {
                curr = node.next.as_mut();
            }
        }

        l.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test203() {
        let list = linkedlist![6,1,1];
        let res = linkedlist![1,1];
        assert_eq!(res, Solution::remove_elements(list, 6));
    }
}