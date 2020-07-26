# rCore 第一阶段总结

## 前言

我是 大连理工大学 18级的一名本科生 ， 在老师的宣传下被 这个项目所深深吸引，自愿加入了进来

在学习之前，我只是对 操作系统有所了解，完成了清华的课程 和 实验

对 `RUST`  ，  `RISC-V`  的了解不是很多，但是 rust 的安全性 到 我们现在在代码中漫天使用的 `spin::Mutex` ， `alloc::sync::Arc`  , 包括自己实现的 `Lock` 都是安全性的体现，在不断学习rust的过程中，对这门语言的了解不断深入，我觉得他借鉴了很多语言



## Rust语言

Rust 的安全性 是我一直觉得rust相较于别的语言突出的地方，我们现在在代码中漫天使用的 `spin::Mutex` ， `alloc::sync::Arc`  , 包括自己实现的 `Lock` 都是安全性的体现，在不断学习rust的过程中，对这门语言的了解不断深入，我觉得他借鉴了很多语言的优势，这也可能是他诞生晚的好处，包括 宏 借鉴了 Lisp 的S表达式等等方面，充分的利用rust 中std库中的函数可以大大减少，代码的数量，甚至在坊间流传 rust 写题可以在一行内完成。我对rust语言印象最深的大致有：

* 所有权（ownership）
* 智能指针
* Option Result 等使得 出现错误时返回值更加严谨，代码的错误率更低，也增加了检错的能力
* macro_rules! 宏的定义，严谨而又花样百出，对抽象语法树有很强的操作能力

### 学习过程

* 阅读 《rust编程之道》 ， 看了 [令狐壹冲的视频](https://www.bilibili.com/video/BV1FJ411Y71o?p=2)  

* 完成了 rustlings 的练习， 弄懂了基本的语法

* 开始着手做 leetcode 上有些难度的习题，学习如何用rust写数据结构，写算法

  * 完成了 7 篇leetcode 的题解，每一篇都有不一样的收获

    [64.最小路径和-滚动数组DP](https://leetcode-cn.com/problems/minimum-path-sum/solution/rust-gun-dong-shu-zu-dp-by-dingiso/)

    [167. 两数之和 II - 输入有序数组-BTreeMap优化](https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/solution/btreemap-by-dingiso/)

    [97. 交错字符串-简单dp](https://leetcode-cn.com/problems/interleaving-string/solution/guan-fang-ti-jie-gai-by-dingiso/)

    [120. 三角形最小路径和-一维动态规划](https://leetcode-cn.com/problems/triangle/solution/la-ji-ti-jie-gun-dong-shu-zu-by-dingiso/)

    [350. 两个数组的交集 II- 垃圾版](https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/solution/zui-la-ji-dai-ma-mei-you-zhi-yi-by-dingiso/)

    [96. 不同的二叉搜索树 - 公式推导](https://leetcode-cn.com/problems/unique-binary-search-trees/solution/jie-ti-si-lu-by-dingiso/)

    [174地下城游戏 - 直观解法](https://leetcode-cn.com/problems/dungeon-game/solution/zhi-guan-jie-fa-by-dingiso/)

* 写了一些 issue

  * [声明周期推断 ](https://github.com/rcore-os/rCore-Tutorial/issues/49#issuecomment-656443196)

  * [lazy_static!](https://github.com/rcore-os/rCore-Tutorial/issues/38#issuecomment-654882048)

  * [谬误](https://github.com/rcore-os/rCore-Tutorial/issues/20#issuecomment-654908994)

    