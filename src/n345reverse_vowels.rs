pub struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        use std::collections::HashSet;
        let v: Vec<char> = s.chars().collect();
        if v.len() <= 1 {
            return s;
        }
        let mut r = Vec::new();
        let mut vowels = HashSet::new();
        vowels.insert('a');
        vowels.insert('e');
        vowels.insert('i');
        vowels.insert('o');
        vowels.insert('u');
        vowels.insert('A');
        vowels.insert('E');
        vowels.insert('I');
        vowels.insert('O');
        vowels.insert('U');
        for item in v.iter() {
            if vowels.contains(item) {
                r.push(item);
            }
        }

        let mut resv = v.clone();
        for item in resv.iter_mut() {
            if vowels.contains(item) {
                *item = *r.pop().unwrap();
            }
        }

        let res: String = resv.iter().collect();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test345() {
        assert_eq!("holle".to_owned(), Solution::reverse_vowels("hello".to_owned()));
        assert_eq!("leotcede".to_owned(), Solution::reverse_vowels("leetcode".to_owned()));
    }   
}