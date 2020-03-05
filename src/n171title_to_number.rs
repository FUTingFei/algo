pub struct Solution {}

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut chars: Vec<char> = s.chars().collect();
        chars.reverse();
        
        let mut num = 0;
        for (i,elem) in chars.iter().enumerate() {
            num += (*elem as u32 - 64) * ((26 as u32).pow(i as u32)); 
        }

        num as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test171() {
        assert_eq!(52, Solution::title_to_number("AZ".to_string()));
        assert_eq!(28, Solution::title_to_number("AB".to_string()));
        assert_eq!(701, Solution::title_to_number("ZY".to_string()));
    }
}