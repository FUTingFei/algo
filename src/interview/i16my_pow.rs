pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res: f64 = 1.0;
        let mut n = n;
        
        if n > 0 {
            for _i in 0..n {
                res = res * x;
            }
        } else {
            n = -n;
            for _i in 0..n {
                res = res * x;
                res = 1.0 / res;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i16() {
        assert_eq!(1024.00000, Solution::my_pow(2.00000, 10));
        assert_eq!(9.26100, Solution::my_pow(2.10000, 3));
        assert_eq!(0.25000, Solution::my_pow(2.00000, -2));
    }
}