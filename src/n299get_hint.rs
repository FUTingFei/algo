pub struct Solution {}

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut s_iter = secret.chars();
        let mut g_iter = guess.chars();
        let mut bucket = vec![0; 10];
        let mut non_match = vec![];
        while let (Some(s), Some(g)) = (s_iter.next(), g_iter.next()) {
            if s == g {
                bulls += 1;
            } else {
                bucket[s.to_digit(10).unwrap() as usize] += 1;
                non_match.push(g.to_digit(10).unwrap() as usize);
            }
        }
        for &idx in non_match.iter() {
            if bucket[idx] > 0 {
                cows += 1;
                bucket[idx] -= 1;
            }
        }
        format!("{}A{}B", bulls, cows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test299() {
        let secret = "1807".to_owned();
        let guess = "7810".to_owned();
        let res = "1A3B".to_owned();
        assert_eq!(res, Solution::get_hint(secret, guess));

        let secret = "1123".to_owned();
        let guess = "0111".to_owned();
        let res = "1A1B".to_owned();
        assert_eq!(res, Solution::get_hint(secret, guess));
    }   
}