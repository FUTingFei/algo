pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test004() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let res1 = 2.0;
        assert_eq!(res1, Solution::find_median_sorted_arrays(nums1, nums2));

        let nums3 = vec![1, 2];
        let nums4 = vec![3, 4];
        let res2 = 2.5;
        assert_eq!(res2, Solution::find_median_sorted_arrays(nums3, nums4));

    }
}