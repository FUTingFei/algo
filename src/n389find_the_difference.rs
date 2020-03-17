pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s:Vec<char> = s.chars().collect();
        let mut t:Vec<char> = t.chars().collect();
        s.sort();
        t.sort();

        let mut f:i32 = -1;
        let mut res = 'a';
        for i in 0..s.len() {
            if s[i] != t[i] {
                res = t[i];
                f = i as i32;
                break;
            }
        }

        if f == -1 {
            res = t[s.len()];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test389() {
        let s = "abcd".to_owned();
        let t = "abcde".to_owned();
        assert_eq!('e', Solution::find_the_difference(s, t));
    }   
}