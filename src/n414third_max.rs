pub struct Solution {}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        let nums_set:HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut res_v: Vec<i32> = Vec::new();

        for elem in nums_set {
            res_v.push(elem);
        }

        res_v.sort_by(|a,b| b.cmp(a));
        
        if res_v.len() < 3 {
            res_v[0]
        } else {
            res_v[2]
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test414() {
        assert_eq!(1, Solution::third_max(vec![3,2,1]));
        assert_eq!(2, Solution::third_max(vec![1,2]));
        assert_eq!(1, Solution::third_max(vec![2,2,3,1]));
    }   
}