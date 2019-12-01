struct Solution {}

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut a = a;
        let r = &a[0].clone();
        let first:Vec<char> = r.chars().collect();
        let mut fs:Vec<String> = Vec::new();
        for i in first {
            fs.push(i.to_string());
        }

        let mut out:Vec<String> = Vec::new();

        for i in fs {
            let mut ax = "0".to_string();
            
            for x in &mut a[1..] {
                if let Some(n) = x.find(i.as_str()) {
                    ax = x.remove(n).to_string();
                } else {
                    ax = "0".to_string();
                    break;
                }
            }

            if ax != "0".to_string() {
                out.push(ax);
            }
        }

        out
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_common_chars() {
        let a:Vec<String> = vec!["cool".to_string(), "lock".to_string(),"cook".to_string()];
        let out = vec!["c".to_string(), "o".to_string()];
        let res = Solution::common_chars(a);
        assert_eq!(out, res);
    }
}