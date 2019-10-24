fn main() {
    let res = roman_to_int("IV".to_owned());
    println!("{:?}", res);
}

pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let v:Vec<char> = s.chars().collect();
    let mut nums:Vec<i32> = Vec::new();
    for i in v {
        let temp = match i {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        nums.push(temp);
    }

    let n = nums.len();
    for i in 0..(n-1) {
        if nums[i as usize] < nums[i as usize + 1] {
            nums[i as usize] = - nums[i as usize];
        }
    }

    for val in nums {
        sum += val;
    }

    sum
}


pub fn is_power_of_three(n: i32) -> bool {
    let mut res = false;

    if n == 1 {
        res = true;
    }

    if n > 3 {
        res = is_three(n);
    }
    
    res
}

fn is_three(n: i32) -> bool {
    let mut res = true;

    if n == 3 {
        return true;
    }

    if n%3 != 0 {
        res = false;
    } else {
        res = is_three(n%3);
    }

    res
}

pub fn count_primes(n: i32) -> i32 {
    let mut vec_bool: Vec<bool> = vec![true; n as usize];

    for i in 2..n {
        if vec_bool[i as usize] {
            let mut j = i * 2;
            while j < n {
                vec_bool[j as usize] = false;
                j = j + i;
            }
        }
    }

    let mut count = 0;
    
    for i in 2..n {
        if vec_bool[i as usize] {
            count += 1;
        }
    }

    count
}


pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut v = Vec::new();

    for i in 1..=n {
        let flag = (i%3 == 0, i%5 == 0);

        let a = match flag {
            (true, false) => "Fizz".to_owned(),
            (false, true) => "Buzz".to_owned(),
            (true, true)  => "FizzBuzz".to_owned(),
            (false, false)=> i.to_string(),
        };

        v.push(a);
    }

    v
}


