struct Solution {}

impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for i in 0..12 {
            for j in 0..60 {
                if Solution::count_one(i) + Solution::count_one(j) == num {
                    res.push(
                        if j < 10 {
                            format!("{}:0{}",i,j)
                        } else {
                            format!("{}:{}",i,j)
                        }
                    )
                }   
            }
        }
        res
    }

    fn count_one(n: i32) -> i32 {
        let mut n = n;
        let mut res = 0;
        while n != 0 {
            n = n & (n - 1);
            res += 1;
        }
        res
    }
}

mod tests {
    use super::*;

    #[test]
    fn test401() {
        let mut res = vec!["1:00".to_owned(), "2:00".to_owned(), "4:00".to_owned(), "8:00".to_owned(), "0:01".to_owned(), "0:02".to_owned(), "0:04".to_owned(), "0:08".to_owned(), "0:16".to_owned(), "0:32".to_owned()];
        assert_eq!(res, Solution::read_binary_watch(1));
    }
}