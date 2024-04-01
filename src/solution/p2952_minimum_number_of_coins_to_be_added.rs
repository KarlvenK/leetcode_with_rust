struct Solution {}

impl Solution {
    pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
        coins.sort_by(|a, b| a.cmp(b));
        let (mut i, mut x) = (0, 1);
        let mut ans = 0;
        // 表示此时[1, x - 1] 可取
        while x <= target {
            if i < coins.len() && coins[i] <= x {
                // 仍有supplied的coin可取且i下标的coin小于x值
                x += coins[i]; // coins[i] + y (1 <= y < x) 属于 [1, x + coins[i] - 1]
                i += 1;
            } else {
                // 此时无法取到x，需要添加新的数，贪心策略，直接加入x，此时区间变为[1, 2x - 1]
                ans += 1;
                x *= 2;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2952() {
        assert_eq!(2, Solution::minimum_added_coins(vec![10, 4, 1], 19));
        assert_eq!(
            1,
            Solution::minimum_added_coins(vec![1, 4, 10, 5, 7, 19], 19)
        );
    }
}
