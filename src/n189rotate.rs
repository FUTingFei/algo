pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let mut k = k;

        if len == 0 || len == 1 {
            return
        } 

        while k > 0 {
            let p = nums.pop().unwrap();
            nums.reverse();
            nums.push(p);
            nums.reverse();
            k -= 1;
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test189() {
        let mut v1 = vec![1,2,3,4,5,6,7];
        let k1 = 3;
        let res1 = vec![5,6,7,1,2,3,4];
        Solution::rotate(&mut v1, k1);
        assert_eq!(res1, v1);

        let mut v2 = vec![-1,-100,3,99];
        let k2 = 2;
        let res2 = vec![3,99,-1,-100];
        Solution::rotate(&mut v2, k2);
        assert_eq!(res2, v2);
    }
}