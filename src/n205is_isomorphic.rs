pub struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let n1 = Solution::to_num(s);
        let n2 = Solution::to_num(t);

        for i in 0..n1.len() {
            if n1[i] != n2[i] {
                return false;
            }
        }

        true
    }

    fn to_num(s: String) -> Vec<i32> {
        use std::collections::HashMap;
        let sv: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut nv: Vec<i32> = Vec::new();
        for (index, c) in sv.iter().enumerate() {
            if map.contains_key(&c) {
                let a = map.get(&c).unwrap();
                nv.push(*a);
            } else {
                nv.push(index as i32);
                map.insert(*c, index as i32);
            }
        }

        nv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test205() {
        let s1 = "egg".to_string();
        let s2 = "add".to_string();
        assert_eq!(true, Solution::is_isomorphic(s1, s2));

        let s1 = "foo".to_string();
        let s2 = "bar".to_string();
        assert_eq!(false, Solution::is_isomorphic(s1, s2));

        let s1 = "paper".to_string();
        let s2 = "title".to_string();
        assert_eq!(true, Solution::is_isomorphic(s1, s2));
    }
}