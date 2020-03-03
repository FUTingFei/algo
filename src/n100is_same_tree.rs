use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p == q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test100() {
        let tree1 = btree![1,2,3];
        let tree2 = btree![1,2,3];
        let res1 = true;
        assert_eq!(res1, Solution::is_same_tree(tree1, tree2));

        let tree3 = btree![1,2];
        let tree4 = btree![1,null,2];
        let res2 = false;
        assert_eq!(res2, Solution::is_same_tree(tree3, tree4));
        
        let tree5 = btree![1,2,1];
        let tree6 = btree![1,1,2];
        let res3 = false;
        assert_eq!(res3, Solution::is_same_tree(tree5, tree6));                
    }
}

