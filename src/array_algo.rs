pub mod array_algo {
    pub fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) {
        let mut temp:i32;
        let l = matrix.len();
        
        for i in 0..(l/2) {
            println!("{}",i);
            println!("{}",l-i-1);
            for j in i..(l-i-1) {
                temp = matrix[i][j];
                matrix[i][j] = matrix[l-j-1][i];
                matrix[l-j-1][i] = matrix[l-i-1][l-j-1];
                matrix[l-i-1][l-j-1] = matrix[j][l-i-1];
                matrix[j][l-i-1] = temp;
            }
        }
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        for line in &board {
            let mut line_set = HashSet::new();

            let mut l = line.clone();
            l.retain(|x| *x != '.');
            // println!("l is {:?}", l);
            let old_len = l.len();
            
            for c in line {
                if *c != '.' {
                    line_set.insert(c);
                }
            }
            let new_len = line_set.len();
            // println!("line_set is {:?}", line_set);
            if new_len != old_len {
                return false;
            }
        }

        for i in 0..9 {
            let mut col_set = HashSet::new();
            let mut col_num = Vec::new();

            for line in &board {
                if line[i] != '.' {
                    col_num.push(line[i]);
                    col_set.insert(line[i]);
                }
            }

            if col_num.len() != col_set.len() {
                return false;
            }
        }

        let mut i = 0;
        
        while i < 9 {
            let mut j = 0;
            while j < 9 {
                let mut box_set = HashSet::new();
                let mut box_num = Vec::new();
                
                for a in i..(i+3) {
                    for b in j..(j+3) {
                        if board[a][b] != '.' {
                            box_num.push(board[a][b]);
                            box_set.insert(board[a][b]);
                        }
                    }
                }

                if box_num.len() != box_set.len() {
                    return false;
                }

                j += 3;
            }

            i += 3;
        }
        
        true
    }
    
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut res = vec![];
        let mut h:HashMap<i32, i32> = HashMap::new();
        
        for (k, n) in nums.iter().enumerate() {
            let key = target - n;
            
            match h.get(&key) {
                Some(v) => {
                    println!("{}", *v);
                    res.push(*v);
                    res.push(k as i32);
                    break;
                },
                None => {
                    h.insert(*n, k as i32);
                }
            }
        }

        res
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        
        let old_len = nums.len();
        nums.retain(|x| *x != 0);
        let new_len = nums.len();
        let s = old_len - new_len;

        let mut i = 0;

        while i < s {
            nums.push(0);
            i += 1;
        }

    }

    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut nums = digits.clone();
        let len = nums.len();
        let mut i = len - 1;

        loop {
            if nums[i] < 9 {
                nums[i] = nums[i] + 1;
                return nums;
            }
            nums[i] = 0;
            if i > 0{
                i = i - 1;
            } else {
                break;
            }
        }

        nums.insert(0, 1);

        nums
    }

    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut result: Vec<i32> = Vec::new();

        let mut i = 0;
        let mut j = 0;
        
        let mut n1 = nums1.clone();
        let mut n2 = nums2.clone();
        
        n1.sort();
        n2.sort();

        while i < len1 && j < len2 {
            if n1[i] == n2[j] {
                result.push(n1[i]);
                i += 1;
                j += 1;
            } else if n1[i] > n2[j] {
                j += 1;
            } else {
                i += 1;
            }
        }

        result   
    }

    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut output = 0;

        for val in nums {
            output = output ^ val;
        }

        output        
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut i = 0;
        for val in &nums {
            let j = i + 1;
            for res in &nums[j..] {
                if val == res {
                    return true;
                }
            }
            i += 1;
        }
        false
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32){
        let mut i = 0;

        while i < k {
            let out = nums.pop().unwrap();
            nums.insert(0, out);
            i += 1;
        }
    }
    
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let mut money = 0;
        let mut buy = prices[0];

        for price in prices {
            if buy < price {
                money += price - buy;
            }
            buy = price;
        }

        money
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut i = 0;

        while i < nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}