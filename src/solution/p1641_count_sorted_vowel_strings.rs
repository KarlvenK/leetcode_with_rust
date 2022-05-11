pub struct Solution {}

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut f = Vec::<Vec<i32>>::new();
        f.resize_with(n, || {
            let mut p = Vec::<i32>::new();
            p.resize(5, 0);
            p
        });

        for i in 0..5 {
            f[0][i] = 1;
        }

        for i in 1..n {
            for j in 0..5 {
                for k in 0..=j {
                    f[i][j] += f[i - 1][k];
                }
            }
        }
        f[n - 1].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1641() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
        assert_eq!(Solution::count_vowel_strings(2), 15);
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }
}
