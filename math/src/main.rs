fn main() {
    let res = is_power_of_three(9);
    println!("{:?}", res);
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


