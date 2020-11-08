pub struct Solution {}

// 递归 + 快速幂

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n > 0 {
            Solution::quick_mul(x, n)
        } else {
            1.0 / Solution::quick_mul(x, n)
        } 
    }

    fn quick_mul(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let y = Solution::quick_mul(x, (n/2).abs());

        if n % 2 == 0 {
            y * y
        } else {
            y * y * x
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i16() {
        assert_eq!(1024.00000, Solution::my_pow(2.00000, 10));
        assert_eq!(9.261000000000001, Solution::my_pow(2.10000, 3));
        assert_eq!(0.25000, Solution::my_pow(2.00000, -2));
    }
}