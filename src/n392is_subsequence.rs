struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t = t.as_str();

        let mut s = s;
        let vs:Vec<char> = s.chars().collect();

        for i in vs {
            let n = t.find(i);
            if let Some(ni) = n {
                let (first, last) = t.split_at(ni as usize + 1);
                t = last;
            } else {
                return false;
                break;
            }
        }

        true
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        let res = Solution::is_subsequence(s, t);
        assert_eq!(res, true);

    }
}