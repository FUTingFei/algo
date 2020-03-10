pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prevMax = 0;
        let mut currMax = 0;

        for val in nums {
            let temp = currMax;
            currMax = i32::max(prevMax + val, currMax);
            prevMax = temp;
        }

        currMax
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test198() {
        let v1 = vec![1,2,3,1];
        assert_eq!(4, Solution::rob(v1));

        let v2 = vec![2,7,9,3,1];
        assert_eq!(12, Solution::rob(v2));
    }
}