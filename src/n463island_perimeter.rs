pub struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let height = grid.len();
        let width = grid[0].len();
        let mut res = 0;

        for i in 0..height {
            for j in 0..width {
                if grid[i][j] == 1 {
                    if i + 1 == height || grid[i+1][j] == 0 {
                        res += 1;
                    }
                    if i as isize - 1 < 0 || grid[i-1][j] == 0 {
                        res += 1;
                    }
                    if j + 1 == width || grid[i][j+1] == 0 {
                        res += 1;
                    }
                    if j as isize - 1 < 0 || grid[i][j-1] == 0 {
                        res += 1;
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test463() {
        let grid = vec![vec![0,1,0,0],vec![1,1,1,0],vec![0,1,0,0], vec![1,1,0,0]];
        assert_eq!(16, Solution::island_perimeter(grid));
    }   
}