use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            Self::helper(&nums, 0, nums.len())
        }
    }

    fn helper(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start >= end {
            None
        } else {
            let mid = (start + end) / 2;
            let node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            node.borrow_mut().left = Self::helper(&nums, start, mid);
            node.borrow_mut().right = Self::helper(&nums, mid+1, end);
            Some(node)   
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test108() {
        let list1 = vec![-10,-3,0,5,9];
        let res = btree![0,-3,9,-10,null,5];
        assert_eq!(res, Solution::sorted_array_to_bst(list1));
    }
}