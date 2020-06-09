pub struct Solution {}

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2
        }

        let mut times_of_3 = n / 3;

        if n - times_of_3 * 3 == 1 {
            times_of_3 -= 1;
        }

        let times_of_2 = (n - times_of_3 * 3)/2;

        let mut t:u128 = 1;
        for _i in 0..times_of_3 {
            t = (t * 3) % 1000000007;
        }

        let s = (2 as u128).pow(times_of_2 as u32) % 1000000007;
        ((t * s) % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i14_2() {
        assert_eq!(473062299, Solution::cutting_rope(255));
        assert_eq!(36, Solution::cutting_rope(10));
    }   
}