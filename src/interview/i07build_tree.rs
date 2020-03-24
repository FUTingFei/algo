use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = preorder.len();
        
        if len == 0 {
            return None;
        }

        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, v) in inorder.iter().enumerate() {
            map.insert(*v, i);
        }

        // 技巧是递归时不用把preorder和inorder两个数组给拆分再递归，可以只传递索引，这样就会避免很多麻烦
        let tree:Option<Rc<RefCell<TreeNode>>> = Solution::get_tree(&preorder, 0, len - 1, &inorder, 0, len - 1, &map);
        
        tree
    }

    fn get_tree(preorder: &Vec<i32>, ps: usize, pe: usize,  inorder:&Vec<i32>, is: usize, ie: usize, map: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if ps > pe {
            return None;
        }

        let root_val = preorder[ps];

        let mut root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));

        if ps == pe {
            return root;
        } else {
            let root_index = map.get(&root_val).unwrap();
            let left_nums = root_index - is;
            let right_nums = ie - root_index;
            let left_subtree = Solution::get_tree(preorder, ps + 1, ps + left_nums, inorder, is, root_index - 1, &map);
            let right_subtree = Solution::get_tree(preorder, pe - right_nums + 1, pe, inorder, root_index + 1, ie, &map);
            root.as_mut().unwrap().borrow_mut().left = left_subtree;
            root.as_mut().unwrap().borrow_mut().right = right_subtree;
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i07() {
        let preorder = vec![3,9,20,15,7];
        let inorder = vec![9,3,15,20,7];
        let res = btree![3,9,20,null,null,15,7];

        assert_eq!(res, Solution::build_tree(preorder, inorder));
    }   
}