pub struct Solution {}
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        fn is_equal(cnt: &Vec<i32>) -> bool {
            let mut last = 0;
            for &c in cnt {
                if c == 0 {
                    continue;
                }
                if last != 0 && last != c {
                    return false;
                }
                last = c;
            }
            true
        }

        let mut cnt = vec![0; 26];
        word.bytes().for_each(|ch| cnt[(ch - b'a') as usize] += 1);
        for i in 0..26 {
            if cnt[i] == 0 {
                continue;
            }
            cnt[i] -= 1;
            if is_equal(&cnt) {
                return true;
            }
            cnt[i] += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2423() {
        assert_eq!(Solution::equal_frequency("abcc".to_string()), true);
        assert_eq!(Solution::equal_frequency("abccc".to_string()), false);
        assert_eq!(Solution::equal_frequency("aabb".to_string()), false);
        assert_eq!(Solution::equal_frequency("abc".to_string()), true);
        assert_eq!(Solution::equal_frequency("aabbccde".to_string()), false);
    }
}
