// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut max_dep = 0;

        fn dfs(dep: i32, cur: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, max_dep: &mut i32) {
            if cur == None {
                return;
            }

            // let node = cur.as_ref().unwrap().borrow_mut();
            let node = cur.unwrap();
            let node = node.borrow_mut();
            if dep > *max_dep {
                *max_dep = dep;
                *ans = node.val;
            } else {
                if dep == *max_dep {
                    *ans += node.val;
                }
            }
            dfs(dep + 1, node.left.clone(), ans, max_dep);
            dfs(dep + 1, node.right.clone(), ans, max_dep);
        }
        dfs(0, root, &mut ans, &mut max_dep);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1302() {
        let mut root = TreeNode::new(0);
        let left = TreeNode::new(10);
        let right = TreeNode::new(11);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));
        assert_eq!(
            Solution::deepest_leaves_sum(Some(Rc::new(RefCell::new(root)))),
            21
        );
    }
}
