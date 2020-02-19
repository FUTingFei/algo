pub struct Solution {}

impl Solution {
    // 1
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res:Vec<Vec<i32>> = Vec::new();
        nums.sort();

        let mut i = 0;
        let mut j = nums.len() - 1;

        for x in 0..nums.len() {
            if x > 0 {
                break;
            }
            let mut i = 0;
            let mut j = nums.len() - 1;
            let mut a = 0;
            let mut sum = 0;
            let mut count = 2;
            let mut re:Vec<i32> = Vec::new();
            re.push(nums[i]);
            re.push(nums[j]);
            sum = nums[i] + nums[j];

            loop {
                if a != 0 {
                    sum += nums[a];
                    re.push(nums[a]);
                    count += 1;
                }

                if count > 3 {
                    re.clear();
                    break;
                } else if count == 3 {
                    if sum == 0 {
                        res.push(re.clone());
                        re.clear();
                        break;
                    } else if sum > 0 {

                    } else {

                    }
                } else { // count < 3
                    if sum > 0 {
                        a = i + 1;
                    } else {
                        a = j - 1;
                    }
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
    fn test1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let answer = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
        let res = Solution::three_sum(nums);
        assert_eq!(res, answer);
    }
}

