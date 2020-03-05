
pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut all: HashMap<i32, i32> = HashMap::new();

        for item in nums {
            if all.contains_key(&item) {
                if let Some(x) = all.get_mut(&item) {
                    *x += 1;
                }
            } else {
                all.insert(item, 1);
            }
        }

        let mut mv = 0;
        let mut mk = 0;
        for (key, val) in all.iter() {
            if *val > mv {
                mv = *val;
                mk = *key;
            }
        }

        mk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test169() {
        let nums1 = vec![3,2,3];
        assert_eq!(3, Solution::majority_element(nums1));
        let nums2 = vec![2,2,1,1,1,2,2];
        assert_eq!(2, Solution::majority_element(nums2));
    }
}