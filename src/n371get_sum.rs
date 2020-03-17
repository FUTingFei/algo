pub struct Solution {}

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 { // 如果还有进位，继续计算
            let carry = (a & b) << 1; // 计算进位
            a = a ^ b; // 不进位相加
            b = carry; // 如果还有进位，就挪到下一轮来计算
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test371() {
        assert_eq!(3, Solution::get_sum(1, 2));
        assert_eq!(1, Solution::get_sum(-1, 2));
    }   
}