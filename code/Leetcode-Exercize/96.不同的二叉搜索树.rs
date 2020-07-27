/**
 * https://leetcode-cn.com/problems/unique-binary-search-trees/
 * 96. 不同的二叉搜索树
给定一个整数 n，求以 1 ... n 为节点组成的二叉搜索树有多少种？

示例:

输入: 3
输出: 5
解释:
给定 n = 3, 一共有 5 种不同结构的二叉搜索树:

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3
 *
 */
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let len: usize = n as usize;
        let mut dp = vec![0; len + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..len + 1 {
            if i % 2 != 0 {
                dp[i] += dp[i / 2] * dp[i - i / 2 - 1];
            }
            for j in 1..i / 2 + 1 {
                dp[i] += 2 * dp[j - 1] * dp[i - j];
            }
        }
        return dp[len];
    }
}
