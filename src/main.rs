// use algo::quicksort::quicksort_lomuto;
use algo::array;

fn main() {
    // let board:Vec<Vec<char>> = vec![
    //     vec!['.','.','.','.','.','.','5','.','.'],
    //     vec!['.','.','.','.','.','.','.','.','.'],
    //     vec!['.','.','.','.','.','.','.','.','.'],
    //     vec!['9','3','.','.','2','.','4','.','.'],
    //     vec!['.','.','7','.','.','.','3','.','.'],
    //     vec!['.','.','.','.','.','.','.','.','.'],
    //     vec!['.','.','.','3','4','.','.','.','.'],
    //     vec!['.','.','.','.','.','3','.','.','.'],
    //     vec!['.','.','.','.','.','5','2','.','.']
    // ];

    // let result = array::is_valid_sudoku(board);
    // println!("{}", result);

    let mut matrix = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9]
    ];

    array::rotate_matrix(&mut matrix);
    println!("{:?}", matrix);
}