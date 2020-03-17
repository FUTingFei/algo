pub struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        use std::iter::FromIterator;

        let set1: HashSet<i32> = HashSet::from_iter(nums1.iter().cloned());
        let set2: HashSet<i32> = HashSet::from_iter(nums2.iter().cloned());

        let mut res:Vec<i32> = Vec::new();

        for s in set1.iter() {
            if set2.contains(s) {
                res.push(*s);
            }
        }
        
        //  题目并不要求输出顺序，但是为了方便我测试，所以排了一下序
        res.sort();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test349() {
        let nums1 = vec![1,2,2,1];
        let nums2 = vec![2,2];
        let res = vec![2];
        assert_eq!(res, Solution::intersection(nums1, nums2));

        let nums1 = vec![4,9,5];
        let nums2 = vec![9,4,9,8,4];
        let res = vec![4,9];
        assert_eq!(res, Solution::intersection(nums1, nums2));
    }   
}