use leetcode_prelude::ListNode;
use leetcode_prelude::linkedlist;
use leetcode_prelude;

struct Solution {}

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        // 获取两个节点
        let (mut l1, mut l2) = (l1, l2);

        // 构造哑结点
        let mut head = Box::new(ListNode::new(0));

        // 新链表尾节点的可变引用，之后操作就用这个随便造，最后再用head来提交就行了
        let mut tail = &mut head;

        while let (Some(node1), Some(node2)) = (l1.as_ref(), l2.as_ref()) {
            if node1.val <=  node2.val {
                // 把较小的这个链表接到新链表尾节点的后面
                tail.next = l1;
                // 将尾节点下一个节点的可变引用作为新尾节点
                tail = tail.next.as_mut().unwrap();
                // 将尾节点之后的链表截取下来重新还给l1
                l1 = tail.next.take();
            } else {
                tail.next = l2;
                tail = tail.next.as_mut().unwrap();
                l2 = tail.next.take();
            }
        }

        tail.next = if l1.is_some() { l1 } else { l2 };

        head.next
    }
    
    // pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    //     let mut head = head;
    //     let len = Solution::len(&head);

    //     if n == 0 {
    //         return head;
    //     } 

    //     let mut temp_head = ListNode::new(0);
    //     temp_head.next = head;

    //     let mut count = 0;
    //     let rn = len - n + 1;

    //     let mut haha = Some(temp_head);


    // }
    
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

    fn len(head: &Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        let mut res = 0;

        while let Some(node) = head {
            res += 1;
            head = &node.next;
        }

        res
    }

}

#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn test_merge() {
        let l1 = linkedlist![1,3,5,7];
        let l2 = linkedlist![2,4,6,8];
        let l3 = linkedlist![1,2,3,4,5,6,7,8];
        let res = Solution::merge_two_lists(l1, l2);
        assert_eq!(l3, res);
    }
}