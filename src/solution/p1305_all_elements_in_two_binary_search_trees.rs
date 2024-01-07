pub struct Solution {}
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

//
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        fn inorder(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(new_node) = node {
                inorder(new_node.borrow_mut().left.take(), ans);
                ans.push(new_node.borrow().val);
                inorder(new_node.borrow_mut().right.take(), ans);
            }
        }
        let mut arr1 = Vec::<i32>::new();
        let mut arr2 = Vec::<i32>::new();
        let mut ans = Vec::<i32>::new();
        inorder(root1, &mut arr1);
        inorder(root2, &mut arr2);

        let (mut i, mut j) = (0_usize, 0_usize);
        while i < arr1.len() && j < arr2.len() {
            if arr1[i] <= arr2[j] {
                ans.push(arr1[i]);
                i += 1;
            } else {
                ans.push(arr2[j]);
                j += 1;
            }
        }

        while i < arr1.len() {
            ans.push(arr1[i]);
            i += 1;
        }
        while j < arr2.len() {
            ans.push(arr2[j]);
            j += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1305() {
        let mut tree1 = TreeNode::new(2);
        tree1.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        tree1.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let root1 = Some(Rc::new(RefCell::new(tree1)));
        let mut tree2 = TreeNode::new(1);
        tree2.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        tree2.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root2 = Some(Rc::new(RefCell::new(tree2)));
        assert_eq!(
            Solution::get_all_elements(root1, root2),
            vec![0, 1, 1, 2, 3, 4]
        );
    }
}
