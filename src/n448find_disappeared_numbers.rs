pub struct Solution {}

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut res:Vec<i32> = Vec::new();
        let mut nums = nums;
        if nums.len() == 0 {
            return nums;
        }

        let len = nums.len();

        for i in 0..len {
            let index = nums[i].abs() as usize - 1;
            if nums[index] > 0 {
                nums[index] = nums[index] * -1;
            }
        }

        for i in 0..len {
            if nums[i] > 0 {
                res.push(i as i32 + 1);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test448() {
        assert_eq!(vec![5,6], Solution::find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]));
    }   
}