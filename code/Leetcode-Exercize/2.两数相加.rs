/**
 * 2。两数相加
 *
给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。

如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例：

输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
原因：342 + 465 = 807
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sum = 0;
        let (mut l1, mut l2) = (l1, l2);
        let mut l = None; // 在上下文中可以推断类型
        let mut p = &mut l;

        loop {
            match (l1, l2) {
                // l1,l2是`move`语义匹配，每次匹配前都需要初始化值
                (Some(v1), Some(v2)) => {
                    sum += v1.val + v2.val;
                    l1 = v1.next;
                    l2 = v2.next;
                }
                (Some(v1), None) => {
                    sum += v1.val;
                    l1 = v1.next;
                    l2 = None;
                }
                (None, Some(v2)) => {
                    sum += v2.val;
                    l2 = v2.next;
                    l1 = None;
                }
                (None, None) => {
                    break;
                }
            }
            *p = Some(Box::new(ListNode::new(sum % 10))); //不管sum是否大于10，都可以使用sum%10的值来构建新“节点“
            sum /= 10; // 获取进位值，否则初始为0
            if let Some(p_box_node) = p {
                p = &mut p_box_node.next
            }
        }
        if sum != 0 {
            *p = Some(Box::new(ListNode::new(sum)));
        }

        l
    }
}
