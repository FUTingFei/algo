use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, nodes: &mut Vec<Vec<i32>>) {
        if node.is_none() {
            return
        }
        while nodes.len() <= depth {
            nodes.push(vec![]);
        }
        let val = node.as_ref().unwrap().borrow().val;
        nodes[depth].push(val);
        Self::dfs(node.as_ref().and_then(|nd| { nd.borrow().left.clone() }), depth+1, nodes);
        Self::dfs(node.as_ref().and_then(|nd| { nd.borrow().right.clone() }), depth+1, nodes);
    }

    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res:Vec<Vec<i32>> = vec![];
        Self::dfs(root, 0, &mut res);
        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test107() {
        let tree = btree![3,9,20,null,null,15,7];
        let res = vec![vec![15,7], vec![9,20], vec![3]];
        assert_eq!(res, Solution::level_order_bottom(tree));
    }
}