pub struct Solution {}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let v:Vec<char> = s.chars().collect();
        let len = v.len();
        if len == 0 {
            return 0;
        }
        
        let mut res = 0;

        for i in 0..len-1 {
            if v[i] != ' ' && v[i + 1] == ' ' {
                res += 1;
            }
        }

        if v[len - 1] != ' ' {
            res + 1
        } else {
            res
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test434() {
        assert_eq!(5, Solution::count_segments("Hello, my name is John".to_owned()));
    }   
}