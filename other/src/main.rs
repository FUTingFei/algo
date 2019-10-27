use std::collections::HashMap;

fn main() {
    let s = 5;
    let res = generate(s);
    println!("{:?}", res);
}

pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let a = x^y;
    let b = format!("{:b}",a);
    let s:Vec<char> = b.chars().collect();
    let mut res = 0;
    for i in s {
        if i == '1' {
            res += 1;
        }
    }
    res
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res:Vec<Vec<i32>> = Vec::new();
    for i in 0..num_rows {
        let mut temp: Vec<i32> = Vec::new();
        if i == 0 {
            temp = vec![1];
        } else {
            let vi = &res[i as usize - 1];
            for j in 0..=i {
                let mut a = 0;
                if j == 0 || j == i {
                    a = 1;
                } else {
                    a = vi[j as usize] + vi[j as usize - 1];
                }
                temp.push(a);
            }
        }
        res.push(temp);
    }

    res
}

pub fn is_valid(s: String) -> bool {
    let vs: Vec<char> = s.chars().collect();
    let len = vs.len();
    if len == 0 {
        return true;
    }

    if len % 2 != 0 {
        return false;
    }

    let mut map: HashMap<char, char> = HashMap::new();
    map.insert(')', '(');
    map.insert(']', '[');
    map.insert('}', '{');

    let mut stack_c: Vec<char> = Vec::new();

    for i in vs {

        if !map.contains_key(&i) {
            stack_c.push(i);
        } else {
            if let Some(a) = stack_c.pop() {
                if a != *map.get(&i).unwrap() {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    if !stack_c.is_empty() {
        return false;
    }
    
    true
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let len = nums.len();
    let mut flag = nums[len as usize - 1];
    let mut i = len;
    if nums[0] != 0 {
        return 0;
    }
    while i > 0 {
        if nums[i as usize - 1] != flag {
            return flag;
        } else {
            i -= 1;
            flag -= 1;
        }
    }
    nums[len as usize - 1] + 1
}