/**
 * 350 . 两个数组的交集 二
 * 给定两个数组，编写一个函数来计算它们的交集。



示例 1：

输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2,2]
示例 2:

输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[4,9]

说明：

输出结果中每个元素出现的次数，应与元素在两个数组中出现次数的最小值一致。
我们可以不考虑输出结果的顺序。
进阶：

如果给定的数组已经排好序呢？你将如何优化你的算法？
如果 nums1 的大小比 nums2 小很多，哪种方法更优？
如果 nums2 的元素存储在磁盘上，磁盘内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/intersection-of-two-arrays-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut r = Vec::new();
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut map1: BTreeMap<i32, i32> = BTreeMap::new();
        let mut map2: BTreeMap<i32, i32> = BTreeMap::new();
        for i in nums1 {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
        for i in nums2 {
            let count2 = map1.entry(i).or_insert(0);
            *count2 += 1;
        }
        for i in map.keys() {
            let c = match map1.get(i) {
                Some(&num) => num.min(*map.get(i).unwrap()),
                None => 0,
            };
            if c > 0 {
                map2.insert(*i, c);
            }
        }
        for (i, j) in map2 {
            for n in 0..j {
                r.push(i);
            }
        }
        return r;
    }
}
/* Java solution
class Solution {
    public int[] intersect(int[] nums1, int[] nums2) {
        if (nums1.length > nums2.length) {
            return intersect(nums2, nums1);
        }
        Map<Integer, Integer> map = new HashMap<Integer, Integer>();
        for (int num : nums1) {
            int count = map.getOrDefault(num, 0) + 1;
            map.put(num, count);
        }
        int[] intersection = new int[nums1.length];
        int index = 0;
        for (int num : nums2) {
            int count = map.getOrDefault(num, 0);
            if (count > 0) {
                intersection[index++] = num;
                count--;
                if (count > 0) {
                    map.put(num, count);
                } else {
                    map.remove(num);
                }
            }
        }
        return Arrays.copyOfRange(intersection, 0, index);
    }
}

作者：LeetCode-Solution
链接：https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/solution/liang-ge-shu-zu-de-jiao-ji-ii-by-leetcode-solution/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/
