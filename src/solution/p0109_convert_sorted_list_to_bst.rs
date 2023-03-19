pub struct Solution {}

use crate::utils::binary_tree::TreeNode;
use crate::utils::lis::ListNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = Vec::new();

        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }

        Solution::solve(&v[..])
    }
    fn solve(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        let left = Solution::solve(&nums[..mid]);
        let right = Solution::solve(&nums[mid + 1..]);
        let mut node = TreeNode::new(nums[mid]);
        node.left = left;
        node.right = right;
        Some(Rc::new(RefCell::new(node)))
    }
}
