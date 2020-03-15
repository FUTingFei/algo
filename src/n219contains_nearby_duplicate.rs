pub struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let len = nums.len();

        for i in 0..len {
            for j in i+1..len {
                if nums[i] == nums[j] && j - i <= k as usize {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test219() {
        let nums = vec![1,2,3,1];
        assert_eq!(true, Solution::contains_nearby_duplicate(nums, 3));

        let nums = vec![1,0,1,1];
        assert_eq!(true, Solution::contains_nearby_duplicate(nums, 1));

        let nums = vec![1,2,3,1,2,3];
        assert_eq!(false, Solution::contains_nearby_duplicate(nums, 2));
    }
}