// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::binary_tree::TreeNode;

struct Solution {}

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = Vec::new();
        if n % 2 == 0 {
            return ans;
        }
        if n == 1 {
            ans.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
            return ans;
        }
        for i in (1..n).step_by(2) {
            let lefts = Self::all_possible_fbt(i);
            let rights = Self::all_possible_fbt(n - i - 1);
            for left_tr in lefts {
                for right_tr in &rights {
                    let root = Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: left_tr.clone(),
                        right: right_tr.clone(),
                    }));
                    ans.push(Some(root));
                }
            }
        }
        ans
    }
}
