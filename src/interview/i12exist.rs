pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        use std::collections::HashSet;
        let mut i:usize = 0;
        let mut j:usize = 0;
        let mut start: Vec<(usize, usize)> = Vec::new();
        let wordv: Vec<char> = word.chars().collect();
        let wl = wordv.len();
        if wl == 0 {
            return true;
        }

        let n = board.len();
        let m = board[0].len();

        for a in 0..n {
            for b in 0..m {
                if board[a][b] == wordv[0] {
                    start.push((a,b));
                }
            }
        }

        for (a, b) in start {
            let mut passed: HashSet<(usize, usize)> = HashSet::new();
            i = a;
            j = b;
            passed.insert((i, j));
            
            let mut curr = 0;

            while curr < wl {
                curr += 1;
                if board[i][j]
            }

            if curr == wl - 1 {
                
            }
            
        }

        true

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i12() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        let word = "ABCCED".to_owned();
        assert_eq!(true, Solution::exist(board, word));

        let board = vec![vec!['a','b'],vec!['c','d']];
        let word = "abcd".to_owned();
        assert_eq!(false, Solution::exist(board, word));
    }   
}