/**
 *
 * https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/
 * 104. 二叉树的最大深度
给定一个二叉树，找出其最大深度。

二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。

说明: 叶子节点是指没有子节点的节点。

示例：
给定二叉树 [3,9,20,null,null,15,7]，

    3
   / \
  9  20
    /  \
   15   7
返回它的最大深度 3
 */
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
use std::cell::RefCell;
use std::rc::Rc;
//static mut IfNone: bool = false;
struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // if let leftNode = root.clone().unwrap().borrow_mut().left.clone(){};
        // let rightNode = root.unwrap().borrow_mut().right.clone();
        // if (leftNode.is_none()) && (rightNode.is_none()) {
        //     return 1;
        // }
        // return Solution::max_depth(leftNode).max(Solution::max_depth(rightNode)) + 1;
        if root.is_none() {
            return 0;
        }
        let mut IfNone = false;
        if let leftNode = root.clone().unwrap().borrow().left.clone() {
            if let rightNode = root.unwrap().borrow().right.clone() {
                return Solution::max_depth(leftNode).max(Solution::max_depth(rightNode)) + 1;
            } else {
                IfNone = true;
                return Solution::max_depth(leftNode) + 1;
            }
        } else {
            if IfNone {
                return 1;
            } else {
                return Solution::max_depth(root.unwrap().borrow().right.clone()) + 1;
            }
        }
    }
}
