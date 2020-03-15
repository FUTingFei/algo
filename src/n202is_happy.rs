pub struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let (mut slow, mut fast) = (n, n);

        loop {
            slow = Solution::compute(slow);
            fast = Solution::compute(fast);
            fast = Solution::compute(fast);
            if slow == fast {
                break;
            }
        }

        slow == 1
    }

    fn compute(n: i32) -> i32 {
        let mut sum = 0;
        let mut n = n;

        while n > 0 {
            let num = n % 10;
            sum += num * num;
            n = n / 10;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test202() {
        assert_eq!(true, Solution::is_happy(1));
    }
}