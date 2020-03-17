use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        Solution::helper(&mut res, root, "".to_owned());

        res
    }

    fn helper(res: &mut Vec<String>, root: Option<Rc<RefCell<TreeNode>>>, path: String) {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                res.push(format!("{}{}", path, node.borrow().val));
            } else {
                let path = format!("{}{}->", path, node.borrow().val);
                Solution::helper(res, node.borrow().left.clone(), path.clone());
                Solution::helper(res, node.borrow().right.clone(), path);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test257() {
        let tree = btree![1,2,3,null,5];
        let res = vec!["1->2->5".to_string(), "1->3".to_string()];
        assert_eq!(res, Solution::binary_tree_paths(tree));
    }
}