pub use leetcode_prelude::ListNode;
pub use leetcode_prelude::linkedlist;
pub use leetcode_prelude;

pub struct Solution {}

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut curr = head.as_ref();

        while let Some(node) = curr {
            res.push(node.val);
            curr = node.next.as_ref();
        }

        res.reverse();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i06() {
        assert_eq!(vec![2,3,1], Solution::reverse_print(linkedlist![1,3,2]));
    }   
}