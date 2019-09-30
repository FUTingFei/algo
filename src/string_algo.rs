pub mod string_algo {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }

    pub fn reverse(x: i32) -> i32 {
        let mut is_minus = false;
        let mut xx;
        if x < 0 {
            xx = -x;
            is_minus = true;
        } else if x == 0 {
            return 0;
        } else {
            xx = x;
        }
        fn remove_zero(x: &mut i32) {
            if *x % 10 == 0 {
                *x = *x / 10;
                remove_zero(x);
            }
        }
        remove_zero(&mut xx);
        let s = xx.to_string();
        let mut c:Vec<char> = s.chars().collect();
        c.reverse();
        let s: String = c.iter().collect();
        let res:Result<i32, _> = s.parse();
        let res = match res {
            Ok(pa) => pa,
            Err(_error) => {
                return 0;
            }
        };
        if is_minus {
            return -res;
        }
        res
    }

    pub fn first_uniq_char(s: String) -> i32 {
        let c:Vec<char> = s.chars().collect();
        let mut dup: Vec<char> = Vec::new();
        let len = c.len();
        if len == 0 {
            return -1;
        }
        if len == 1 {
            return 0;
        }
        for i in 0..(len-1) {
            if !c[(i+1)..].contains(&c[i]) && !dup.contains(&c[i]) {
                return i as i32;
            } else {
                dup.push(c[i]);
            }
        }
        if !dup.contains(&c[len - 1]) {
            return (len - 1) as i32;
        }
        -1
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_char: Vec<char> = s.chars().collect();
        let mut t_char: Vec<char> = t.chars().collect();
        let s_len = s_char.len();
        let t_len = t_char.len();
        if s_len != t_len {
            return false;
        }
        s_char.sort();
        t_char.sort();
        for i in 0..s_len {
            if s_char[i] != t_char[i] {
                return false;
            }
        }
        true
    }

    pub fn is_palindrome(s: String) -> bool {
        let char_list: Vec<char> = vec![
            'a', 'b', 'c', 'd', 'e', 
            'f', 'g', 'h', 'i', 'j', 
            'k', 'l', 'm', 'n', 'o',
            'p', 'q', 'r', 's', 't', 
            'u', 'v', 'w', 'x', 'y', 
            'z', 'A', 'B', 'C', 'D',  
            'F', 'G', 'H', 'I', 'J', 
            'K', 'L', 'M', 'N', 'O',
            'P', 'Q', 'R', 'S', 'T', 
            'U', 'V', 'W', 'X', 'Y', 
            'Z', 'E',
        ];
        let number_list:Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9'];
        let mut c: Vec<char> = s.to_lowercase().chars().collect();
        let mut t = c.clone();
        let mut len = c.len();
        if len == 0 {
            return true;
        }
        t.reverse();
        c.retain( |&x| char_list.contains(&x) || number_list.contains(&x) );
        t.retain( |&x| char_list.contains(&x) || number_list.contains(&x) );
        len = c.len();
        println!("{:?}", c);
        println!("{:?}", t);
        for i in 0..len {
            if c[i] != t[i] {
                return false;
            }
        }
        true
    }

    pub fn my_atoi(str: String) -> i32 {
        let number_list:Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9'];
        let mut str_vec: Vec<char> = str.chars().collect();
        let len_str = str_vec.len();
        if len_str == 0 {
            return 0;
        }
        let mut res_vec: Vec<char> = vec![];
        let mut i = 0;
        for l in &str_vec {
            if *l == ' ' {
                if len_str == 1 {
                    return 0;
                }
                i += 1;
            } else {
                break;
            }
        }
        for _k in 0..i {
            str_vec.remove(0);
        }
        let mut is_minus = false;
        if str_vec[0] == '-' {
            if len_str == 1 {
                return 0;
            }
            is_minus = true;
            str_vec.remove(0);
        }
        if !number_list.contains(&str_vec[0]) {
            return 0;
        }
        for l in &str_vec {
            if number_list.contains(l) {
                res_vec.push(*l);
            } else {
                break;
            }
        }
        let res_str: String = res_vec.iter().collect();
        let res: Result<i32, _> = res_str.parse();
        let res = match res {
            Ok(value) => {
                if is_minus {
                    -value
                } else {
                    value
                }
            },
            Err(_error) => {
                if is_minus {
                    -2147483648
                } else {
                    2147483647
                }
                
            }
        };
        res
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h: Vec<char> = haystack.chars().collect();
        let n: Vec<char> = needle.chars().collect();
        let h_len = h.len();
        let n_len = n.len();
        if n_len == 0{
            return 0;
        }
        if h_len < n_len {
            return -1;
        }
        let mut target = 0;
        let mut index = 0;
        for j in 0..h_len {
            for i in 0..n_len {
                if n[i] == h[j] {
                    target += 1;
                    index = j;
                } else {
                    if target != 0 {
                        target -= 1;
                    }
                }
            }
            if target == n_len {
                return (index) as i32;
            }
        }
        -1
    }

    pub fn count_and_say(n: i32) -> String {
        use std::char;

        fn read(s: String) -> String {
            let arr: Vec<char> = s.chars().collect();
            println!("{:?}", arr);
            let len = arr.len();
            let mut res: Vec<char> = vec![];
            let mut count:u32 = 1;
            for i in 0..len {
                if i == len - 1 {
                    let c = char::from_digit(count, 10);
                    let c = match c {
                        Some(value) => value,
                        None => '-',
                    };
                    if c == '-' {
                        panic!("the char is not a number");
                    }
                    res.push(c);
                    res.push(arr[i]);
                    break;
                }

                if arr[i] != arr[i + 1] {
                    let c = char::from_digit(count, 10);
                    let c = match c {
                        Some(value) => value,
                        None => '-',
                    };
                    if c == '-' {
                        panic!("the char is not a number");
                    }
                    res.push(c);
                    res.push(arr[i]);
                    count = 1;
                } else {
                    count += 1;
                }
            }
            let res:String = res.iter().collect();
            res
        }

        if n == 1 {
            "1".to_string()
        } else {
            read(count_and_say(n - 1))
        }
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        use std::collections::HashSet;
        let mut pool: HashSet<char> = HashSet::new();
        let mut res:Vec<char> = Vec::new();

        let mut len = strs[0].len();
        for str in &strs {
            if str.len() < len {
                len = str.len();
            }
        }

        if len == 0 {
            return "".to_string();
        }

        for i in 0..len {
            for str in &strs {
                let arr: Vec<char> = str.chars().collect();
                pool.insert(arr[i]);
            }
            println!("{:?}", pool);
            if pool.len() == 1 {
                let mut sd: char = ' ';
                for x in pool.iter() {
                    sd = *x;
                }
                res.push(sd);
                pool.clear();
            } else {
                break;
            }
        }

        if res.len() == 0 {
            return "".to_string();
        }

        let res:String = res.iter().collect();
        res
    }
}
