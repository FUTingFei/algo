pub struct Solution {}

impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut ans = String::from("");

        for c in s.as_str().chars() {
            if c == ' ' {
                ans += "%20";
            } else {
                ans.insert(ans.len(), c);
            }
        }

        ans
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i05() {
        assert_eq!("We%20are%20happy.".to_owned(), Solution::replace_space("We are happy.".to_owned()));
    }   
}