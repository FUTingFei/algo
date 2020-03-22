pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let sv: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();

        if sv.len() == 0 {
            return 0;
        }

        if sv.len() == 1 {
            return 1;
        }

        for elem in sv.iter() {
            if map.contains_key(elem) {
                *map.get_mut(elem).unwrap() += 1;
            } else {
                map.insert(*elem, 1);
            }
        }

        let mut res = 0;
        let mut has_odd = false;

        for (_, val) in map.iter() {
            if val % 2 == 0 {
                res += *val;
            } else {
                has_odd = true;
                res += *val - 1;
            }
        }

        if has_odd {
            res + 1
        } else {
            res
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test409() {
        assert_eq!(983, Solution::longest_palindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".to_owned()));
    }   
}