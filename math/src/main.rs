fn main() {
    let vec = fizz_buzz(15);
    println!("{:?}", vec);
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