pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let pattern: Vec<char> = pattern.chars().collect();
        let s:Vec<&str> = str.as_str().split_whitespace().collect();
        let pattern = Solution::helper(pattern);
        let s = Solution::helper(s);
        if pattern.len() != s.len() {
            return false;
        } else {
            for i in 0..pattern.len() {
                if pattern[i] != s[i] {
                    return false;
                }
            }
        }
        true
    }

    fn helper<T>(v: Vec<T>) -> Vec<usize>
        where T: std::cmp::Eq + std::hash::Hash + Copy {
        use std::collections::HashMap;
        let mut res:Vec<usize> = Vec::new();
        let mut map: HashMap<T, usize> = HashMap::new();
        for (i,c) in v.iter().enumerate() {
            if !map.contains_key(c) {
                map.insert(*c, i);
                res.push(i);
            } else {
                let a = *map.get(c).unwrap();
                map.insert(*c, a);
                res.push(a);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test290() {
        let pattern = "abba".to_owned();
        let str = "dog cat cat dog".to_owned();
        assert_eq!(true, Solution::word_pattern(pattern, str));

        let pattern = "abba".to_owned();
        let str = "dog cat cat fish".to_owned();
        assert_eq!(false, Solution::word_pattern(pattern, str));

        let pattern = "aaaa".to_owned();
        let str = "dog cat cat dog".to_owned();
        assert_eq!(false, Solution::word_pattern(pattern, str));

        let pattern = "abba".to_owned();
        let str = "dog dog dog dog".to_owned();
        assert_eq!(false, Solution::word_pattern(pattern, str));
    }
}