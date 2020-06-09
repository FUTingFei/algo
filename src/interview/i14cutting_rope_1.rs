pub struct Solution {}

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }

        let mut products: Vec<i32> = Vec::new();
        products.push(0);
        products.push(1);
        products.push(2);
        products.push(3);

        let mut max;

        for i in 4..=n {
            max = 0;
            for j in 1..=(i/2) {
                let product = products[j as usize] * products[(i-j) as usize];
                if product > max {
                    max = product;
                }
            }
            products.push(max);
        }

        products.pop().unwrap()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i14() {
        assert_eq!(1, Solution::cutting_rope(2));
        assert_eq!(36, Solution::cutting_rope(10));
    }   
}