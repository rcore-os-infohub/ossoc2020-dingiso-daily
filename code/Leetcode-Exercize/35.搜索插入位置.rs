/**https://leetcode-cn.com/problems/search-insert-position/
 * 给定一个排序数组和一个目标值，在数组中找到目标值，
 * 并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置
 */
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut r = 0;
    let len = nums.len();
    for i in 0..len {
        if target >= nums[i] {
            r = i;
        }
    }
    if target > nums[r] {
        r = r + 1;
    }
    return r as i32;
}
