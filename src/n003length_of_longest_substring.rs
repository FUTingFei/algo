pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let v:Vec<char> = s.chars().collect();
        let n = v.len();

        let (mut i, mut j, mut res) = (0,0,0);

        while i < n && j < n {
            if !&v[i..j].contains(&v[j]) {
                res = i32::max(res, (j-i+1) as i32);
                j += 1;
            } else {
                i += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test003() {
        assert_eq!(3, Solution::length_of_longest_substring("dvdf".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
        assert_eq!(4, Solution::length_of_longest_substring("pwwkec".to_string()));
    }
}