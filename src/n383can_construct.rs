pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::HashMap;
        let rv: Vec<char> = ransom_note.chars().collect();
        let mv: Vec<char> = magazine.chars().collect();

        let mut rmap: HashMap<char, i32> = HashMap::new();
        let mut mmap: HashMap<char, i32> = HashMap::new();

        for item in rv {
            if let Some(n) = rmap.get_mut(&item) {
                *n += 1;
            } else {
                rmap.insert(item, 1);
            }
        }

        for item in mv {
            if let Some(n) = mmap.get_mut(&item) {
                *n += 1;
            } else {
                mmap.insert(item, 1);
            }
        }

        for (rk, rv) in rmap.iter_mut() {
            while *rv > 0 {
                if let Some(n) = mmap.get_mut(rk) {
                    if *n > 0 {
                        *n -= 1;
                        *rv -= 1;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test383() {
        assert_eq!(false, Solution::can_construct("a".to_owned(), "b".to_owned()));
        assert_eq!(false, Solution::can_construct("aa".to_owned(), "ab".to_owned()));
        assert_eq!(true, Solution::can_construct("aa".to_owned(), "aab".to_owned()));
    }   
}