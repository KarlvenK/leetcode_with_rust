use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![-1; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount as usize {
            for j in 0..coins.len() {
                if i < coins[j] as usize {
                    continue;
                }
                if i == coins[j] as usize {
                    dp[i] = 1;
                    break;
                }
                if dp[i - coins[j] as usize] != -1 {
                    if dp[i] == -1 {
                        dp[i] = dp[i - coins[j] as usize] + 1;
                    } else {
                        dp[i] = min(dp[i], dp[i - coins[j] as usize] + 1)
                    }
                }
            }
        }
        dp[amount as usize]
    }
}
