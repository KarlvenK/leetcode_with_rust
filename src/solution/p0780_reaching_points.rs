pub struct Solution {}

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let (mut x, mut y) = (tx, ty);
        while x > sx && y > sy && x != y {
            if x > y {
                x %= y;
            } else {
                y %= x;
            }
        }
        if x == sx && y == sy {
            return true;
        }
        if x == sx {
            return y > sy && (y - sy) % x == 0;
        }
        if y == sy {
            return x > sx && (x - sx) % y == 0;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_780() {
        assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
        assert_eq!(Solution::reaching_points(1, 1, 2, 2), false);
        assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
        assert_eq!(Solution::reaching_points(1, 2, 999999999, 2), true);
    }
}
