pub struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut num = vec![0; 11];
        for i in 1..=9 {
            if i + k > 9 && i - k < 0 {
                continue;
            }
            num[1] = i;
            Solution::dfs(&mut ans, &mut num, 2, n, k);
        }
        ans
    }
    fn dfs(ans: &mut Vec<i32>, num: &mut Vec<i32>, index: usize, n: i32, k: i32) {
        if index == n as usize + 1 {
            let mut res = 0;
            for i in 1..=n {
                res = res * 10 + num[i as usize];
            }
            ans.push(res);
            return;
        }
        if num[index - 1] + k <= 9 {
            num[index] = num[index - 1] + k;
            Solution::dfs(ans, num, index + 1, n, k);
        }
        if k == 0 {
            return;
        }
        if num[index - 1] - k >= 0 {
            num[index] = num[index - 1] - k;
            Solution::dfs(ans, num, index + 1, n, k);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0967() {
        assert_eq!(
            Solution::nums_same_consec_diff(3, 7).sort(),
            vec![181, 292, 707, 818, 929].sort()
        );
        assert_eq!(
            Solution::nums_same_consec_diff(2, 1).sort(),
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98].sort()
        );
        println!("{:?}", Solution::nums_same_consec_diff(2, 0));
    }
}
