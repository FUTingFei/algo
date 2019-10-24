use leetcode_prelude::ListNode;
use leetcode_prelude;

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ol = head;
        let mut nl = None;

        while let Some(mut node) = ol {
            ol = node.next;
            node.next = nl;
            nl = Some(node)
        }

        nl
    }
}

#[cfg(test)]
mod test {
    use super::*;
}