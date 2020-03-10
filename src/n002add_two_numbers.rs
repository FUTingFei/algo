use leetcode_prelude::ListNode;
use leetcode_prelude::linkedlist;
use leetcode_prelude;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut nums1: Vec<i32> = Vec::new();
        let mut nums2: Vec<i32> = Vec::new();

        while let Some(node) = l1.as_mut() {
            nums1.push(node.val);
            l1 = node.next.take();
        }

        while let Some(node) = l2.as_mut() {
            nums2.push(node.val);
            l2 = node.next.take();
        }

        nums1.reverse();
        nums2.reverse();

        let mut resv:Vec<i32> = Vec::new();
        let mut flag = true;

        while !(nums1.is_empty() && nums2.is_empty()) {
            let n1 = if let Some(num) = nums1.pop() {num} else {0}; 
            let n2 = if let Some(num) = nums2.pop() {num} else {0}; 

            let r = if flag {
                n1+n2
            } else {
                n1+n2+1
            };

            if r >= 10 {
                resv.push(r%10);
                flag = false;
            } else {
                resv.push(r);
                flag = true;
            }
        }

        if !flag {
            resv.push(1);
        }

        resv.reverse();

        let mut res:Option<Box<ListNode>> = None;

        for item in resv {
            let mut next = ListNode::new(item);
            next.next = res;
            res = Some(Box::new(next));
        }

        res
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test002() {
        let list1 = linkedlist![5];
        let list2 = linkedlist![5];
        let res = linkedlist![0,1];
        assert_eq!(res, Solution::add_two_numbers(list1,list2));
    }

}