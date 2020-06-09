pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let word: Vec<char> = word.chars().collect();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Solution::dfs(&mut board, &word, i as i32, j as i32, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, word: &Vec<char>, i: i32, j: i32, k: usize) -> bool {
        if i >= board.len() as i32
        || i < 0
        || j >= board[0].len() as i32
        || j < 0 
        || board[i as usize][j as usize] != word[k] {
            return false;
        }

        if k == word.len() - 1 {
            return true;
        }

        let temp = board[i as usize][j as usize];
        board[i as usize][j as usize] = '/';

        let res =   Solution::dfs(board, word, i + 1, j, k + 1)
                ||  Solution::dfs(board, word, i - 1, j, k + 1)
                ||  Solution::dfs(board, word, i, j + 1, k + 1)
                ||  Solution::dfs(board, word, i, j - 1, k + 1);
        
        board[i as usize][j as usize] = temp;
        
        res
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