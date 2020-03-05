pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let len = numbers.len();

        for i in 0..len {
            for j in i+1..len {
                if numbers[i] + numbers[j] == target {
                    res.push(i as i32 + 1);
                    res.push(j as i32 + 1);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test167() {
        let nums = vec![2,3,4];
        let target = 6;
        assert_eq!(vec![1,3], Solution::two_sum(nums, target));
    }
}