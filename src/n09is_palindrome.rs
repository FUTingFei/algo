pub fn is_palindrome(x: i32) -> bool {

    if x < 0 {
        return false;
    }

    let x: Vec<char> = x.to_string().chars().collect();

    let n = x.len();

    for i in 0..n {
        if x[i] != x[n-i-1] {
            return false;
        }
    }

    true
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1 {
        let (x,y,z) = (123, 121, -10);
        let res1 = is_palindrome(x);
        let res2 = is_palindrome(y);
        let res3 = is_palindrome(z);

        assert_eq!(res1, false);
        assert_eq!(res2, true);
        assert_eq!(res3, false);

    }
}
