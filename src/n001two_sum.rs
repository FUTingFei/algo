pub struct Solution {}

impl Solution {
    
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut map = std::collections::HashMap::new();

        for (k, n) in nums.iter().enumerate() {
            let key = target - n;
            if let Some(j) = map.get(&key) {
                res.push(*j as i32);
                res.push(k as i32);
                break;
            } else {
                map.insert(*n, k as i32);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn n001() {
        let v = vec![3,2,4];
        let answer = vec![1, 2];
        assert_eq!(Solution::two_sum(v, 6), answer);
    }
}