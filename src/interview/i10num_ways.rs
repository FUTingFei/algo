pub struct Solution {}

impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        let mut v:Vec<i128> = Vec::new();
        v.push(0);
        v.push(1);
        for i in 2..n+2 {
            v.push(v[i as usize -1] + v[i as usize - 2]);
        }
        if n == 0 {
            return 1;
        }
        (v.pop().unwrap() % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i10_2() {
        assert_eq!(2, Solution::num_ways(2));
        assert_eq!(21, Solution::num_ways(7));
    }   
}