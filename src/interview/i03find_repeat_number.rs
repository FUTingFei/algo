pub struct Solution {}

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut res = 0;

        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1] {
                res = nums[i];
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i03() {
        assert_eq!(3, Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]));
    }   
}