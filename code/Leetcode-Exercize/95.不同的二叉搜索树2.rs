/**https://leetcode-cn.com/problems/unique-binary-search-trees-ii/
 * 95. 不同的二叉搜索树 II
给定一个整数 n，生成所有由 1 ... n 为节点所组成的 二叉搜索树 。



示例：

输入：3
输出：
[
  [1,null,3,2],
  [3,2,null,1],
  [3,1,null,null,2],
  [2,1,3],
  [1,null,2,null,3]
]
解释：
以上的输出对应以下 5 种不同结构的二叉搜索树：

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3


提示：

0 <= n <= 8
 *
 *
 */
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
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        Solution::gen_trees(1, n)
    }
    fn gen_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }
        let mut allTrees = Vec::new();
        for i in start..=end {
            let leftTrees = Solution::gen_trees(start, i - 1);
            let rightTrees = Solution::gen_trees(i + 1, end);
            for l in &leftTrees {
                for r in &rightTrees {
                    let currentTree = Rc::new(RefCell::new(TreeNode::new(i)));
                    let leftNode = currentTree.borrow_mut().left = l.clone();
                    let rightNode = currentTree.borrow_mut().right = r.clone();
                    allTrees.push(Some(currentTree));
                }
            }
        }
        allTrees
    }
}
