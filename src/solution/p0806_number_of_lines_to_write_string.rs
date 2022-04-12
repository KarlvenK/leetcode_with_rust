pub struct Solution {}

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut ans = vec![0; 2];
        let mut cnt = 0;

        for &ch in s.as_bytes() {
            // print!("{}", ch as char);
            let sz = widths[(ch - 'a' as u8) as usize];
            if cnt + sz > 100 {
                ans[0] += 1;
                cnt = sz;
            } else {
                if cnt + sz == 100 {
                    ans[0] += 1;
                    cnt = 0
                } else {
                    cnt += sz;
                }
            }
        }
        if cnt == 0 {
            ans[1] = 100
        } else {
            ans[0] += 1;
            ans[1] = cnt;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_806() {
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                String::from("abcdefghijklmnopqrstuvwxyz")
            ),
            vec![3, 60]
        );

        assert_eq!(
            Solution::number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                String::from("bbbcccdddaaa")
            ),
            vec![2, 4]
        );
    }
}
