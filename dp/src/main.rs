fn main() {
    let res = climb_stairs(44);
    println!("{}", res);
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut arr:Vec<i32> = Vec::with_capacity(n as usize + 1);
    arr.push(1);
    arr.push(1);
    for i in 2..=n {
        arr.push(arr[i as usize - 1] + arr[i as usize - 2]);
    }
    arr[n as usize] as i32
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut profit = 0;

    for i in 0..len {
        for j in (i+1)..len {
            if prices[j] - prices[i] > profit  {
                profit = prices[j] - prices[i];
            }
        }
    }

    profit
}


pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        
    1
}
