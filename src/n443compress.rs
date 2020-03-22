pub struct Solution {}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        use std::char;
        let (mut i, mut j) = (0, 1);
        let mut count:u32 = 0;

        loop {
            if j > chars.len() - 1 {
                break;
            }
            if chars[i] != chars[j] {
                i = j;
                j += 1;
                count = 0;
            } else {
                count += 1;
                if count == 1 {
                    chars[j] = char::from_digit(count + 1, 10).unwrap();
                    j += 1;
                } else if count < 9 {
                    chars[i+1] = char::from_digit(count + 1, 10).unwrap();
                    chars.remove(j);
                } else if count == 9 {
                    chars[i+1] = '1';
                    chars[i+2] = '0';
                    j += 1;
                } else if count < 99 {
                    let s = char::from_digit((count + 1)%10, 10).unwrap();
                    let f = char::from_digit((count + 1)/10, 10).unwrap();
                    chars[i+1] = f;
                    chars[i+2] = s;
                    chars.remove(j);
                }  else if count == 99 {
                    chars[i+3] = '1';
                    chars[i+2] = '0';
                    chars[i+1] = '0';
                    j += 1;
                } else if count < 999{
                    let t = char::from_digit((count + 1)%10, 10).unwrap();
                    let s = char::from_digit(((count + 1)/10)%10, 10).unwrap();
                    let f = char::from_digit((count + 1)/100, 10).unwrap();
                    chars[i+3] = t;
                    chars[i+2] = s;
                    chars[i+1] = f;
                } else {
                    chars[i+4] = '0';
                    chars[i+3] = '0';
                    chars[i+2] = '0';
                    chars[i+1] = '1';
                }
            }
        }

        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test443() {
        let mut chars = vec!['a','a','b','b','c','c','c'];
        assert_eq!(6, Solution::compress(&mut chars));

        let mut chars = vec!['a'];
        assert_eq!(1, Solution::compress(&mut chars));

        let mut chars = vec!['a','b','b','b','b','b','b','b','b','b','b','b','b'];
        assert_eq!(4, Solution::compress(&mut chars));
    }   
}