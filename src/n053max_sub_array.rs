pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut sum = 0;

    for item in nums {
        if sum > 0 {
            sum = sum + item;
        } else {
            sum = item;
        }
        res = std::cmp::max(sum, res);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        let res = max_sub_array(nums);
        assert_eq!(res, 6);
    }

    #[test]
    fn test2() {
        let nums = vec![-2,-1];
        let res = max_sub_array(nums);
        assert_eq!(res, -1);
    }

}