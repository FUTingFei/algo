pub struct Solution {}

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut n = n;
        let mut nums:Vec<char> = Vec::new();

        loop {
            if n > 26 {
                let x = (n % 26) as u8;
                if x == 0 {
                    nums.push('Z');
                    n = n / 26 - 1;
                } else {
                    nums.push((x+64) as char);
                    n = n / 26;
                }
            } else {
                nums.push((n+64) as u8 as char);
                break;
            }
        }
        
        nums.reverse();
        nums.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test168() {
        assert_eq!("BZ", Solution::convert_to_title(78));
        // assert_eq!("AB", Solution::convert_to_title(28));
        // assert_eq!("ZY", Solution::convert_to_title(701));
    }
}