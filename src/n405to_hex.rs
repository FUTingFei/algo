pub struct Solution {}

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut num:u32 = num as u32;
        if num == 0 {
            return "0".to_owned();
        }
        let hexv: Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
        
        let mut resv: Vec<char> = Vec::new();

        while num != 0 {
            let r = hexv[num as usize % 16];
            resv.push(r);
            num = num / 16;
        }

        resv.reverse();
        let res: String = resv.iter().collect();
        res 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test405() {
        assert_eq!("1a", Solution::to_hex(26));
        assert_eq!("ffffffff", Solution::to_hex(-1));
    }   
}