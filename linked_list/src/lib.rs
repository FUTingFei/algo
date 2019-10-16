// use leetcode_prelude::ListNode;
// use leetcode_prelude;




#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 找到倒数第n+1个
        let mut len = 0;
        // Solution::len(&head, &mut len);

        // 将 next 指向倒数第n-1个或者NULL

        head
    }

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
    #[test]
    fn test_remove_nth() {
        let list = leetcode_prelude::linkedlist![1,2,3];
        let list_temp = leetcode_prelude::linkedlist![1,2,3];
        let res = Solution::remove_nth_from_end(list, 2);
        assert_eq!(res, list_temp);
    }
}