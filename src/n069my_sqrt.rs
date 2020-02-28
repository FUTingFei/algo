pub fn my_sqrt(x: i32) -> i32 {
    let mut i:i128 = 0;
    loop {
        if i*i <= x as i128 && (i+1)*(i+1) > x as i128  {
            return i as i32 ;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let x = 2147395600;
        let res = my_sqrt(x);
        assert_eq!(46340, res);
    }
}