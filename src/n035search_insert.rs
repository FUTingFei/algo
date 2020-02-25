pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {

    let mut re = 0;
    let len = nums.len();
    let mut i = 0;

    loop {

        if i <= len - 1 {

            if nums[i] >= target {
                re = i;
                break;
            } else {
                i += 1;
            }

        } else {
            re = len;
            break;
        }

    }

    re as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1,3,5,6];
        let val = 5;
        let res = search_insert(nums, val);
        assert_eq!(2,res);
    }

    #[test]
    fn test2() {
        let nums = vec![1,3,5,6];
        let val = 2;
        let res = search_insert(nums, val);
        assert_eq!(1,res);
    }

    #[test]
    fn test3() {
        let nums = vec![1,3,5,6];
        let val = 7;
        let res = search_insert(nums, val);
        assert_eq!(4,res);
    }

    #[test]
    fn test4() {
        let nums = vec![1,3,5,6];
        let val = 0;
        let res = search_insert(nums, val);
        assert_eq!(0,res);
    }
}