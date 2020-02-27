pub fn length_of_last_word(s: String) -> i32 {
    if s == String::from("") {
        return 0;
    }
    let mut s:Vec<char> = s.chars().collect();
    let mut res = 0;
    s.reverse();

    let mut i = 0;
    loop {
        if s.len() == 0 {
            return 0;
        }
        if s[0] == ' ' {
            i += 1;
            s.remove(0);
        } else {
            break;
        }
    }

    let mut j = 0;
    loop {
        if s[j] == ' ' {
            res = j;
            break;
        } else {
            j += 1;
        }

        if j == s.len() {
            res = j;
            break;
        }
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from(" ");
        let res = length_of_last_word(s);
        assert_eq!(0, res);
    }
}