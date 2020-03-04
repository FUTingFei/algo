use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(root.as_ref()).is_some()
    }

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let pair = (
                Solution::helper(node.borrow().left.as_ref()),
                Solution::helper(node.borrow().right.as_ref()),
            );
            match pair {
                (Some(left), Some(right)) => {
                    if i32::abs(left - right) < 2 {
                        return Some(i32::max(left, right) + 1);
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        } else {
            return Some(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test110() {
        let tree1 = btree![3,9,20,null,null,15,7];
        assert_eq!(true, Solution::is_balanced(tree1));

        let tree2 = btree![1,2,2,3,3,null,null,4,4];
        assert_eq!(false, Solution::is_balanced(tree2));

    }
}