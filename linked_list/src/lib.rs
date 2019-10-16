use leetcode_prelude::ListNode;
use leetcode_prelude;

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_node: ListNode = ListNode::new(0);
        dummy_node.next = head;
        let mut dummy = Some(Box::new(dummy_node));
        let mut len = -1;
        let mut ol = &dummy;

        while let Some(node) = ol {
            len += 1;
            ol = &node.next;
        }

        let mut real = &mut dummy;
        let mut count = len - n;
        while count > 0 {
            count -= 1;
            if let Some(node) = real {
                real = &mut node.next;
            }
        }
        
        if let Some(mut node) = real {
            if let Some(n_node) = &node.next {
                node.next = n_node.next;
            }
        }


        dummy.unwrap().next
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