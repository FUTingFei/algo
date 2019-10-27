use std::collections::HashMap;

fn main() {
    let s = "({}[]())".to_owned();
    let res = is_valid(s);
    println!("{}", res);
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