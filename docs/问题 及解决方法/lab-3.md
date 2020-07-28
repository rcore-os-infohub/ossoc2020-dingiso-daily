# lab-3

```bash
root@KaDingisoul:~/lab-3-0/os# make run
   Compiling algorithm v0.1.0 (/root/lab-3-0/os/src/algorithm)
   Compiling os v0.1.0 (/root/lab-3-0/os)
error: cannot find macro `vec` in this scope
  --> src/memory/mapping/memory_set.rs:21:24
   |
21 |         let segments = vec![
   |                        ^^^

error[E0412]: cannot find type `VecDeque` in this scope
  --> src/memory/mapping/mapping.rs:25:19
   |
25 |     mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
   |                   ^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
6  | use alloc::collections::VecDeque;
   |
# mapping.rs 加入 use alloc::collections::VecDeque;
error[E0433]: failed to resolve: use of undeclared type or module `VecDeque`
  --> src/memory/mapping/mapping.rs:36:27
   |
36 |             mapped_pairs: VecDeque::new(),
   |                           ^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
6  | use alloc::collections::VecDeque;
   |

error[E0412]: cannot find type `Mapping` in this scope
 --> src/memory/mapping/memory_set.rs:4:18
  |
4 |     pub mapping: Mapping,
  |                  ^^^^^^^ not found in this scope
  |
help: consider importing this struct
  |
1 | use crate::memory::mapping::Mapping;
  |                                                                                              
                                                                                                 
error[E0412]: cannot find type `Vec` in this scope
 --> src/memory/mapping/memory_set.rs:6:19                                                       
  |
6 |     pub segments: Vec<Segment>,
  |                   ^^^ not found in this scope
  |
help: consider importing one of these items
  |
1 | use alloc::vec::Vec;
  |
1 | use crate::memory::mapping::mapping::Vec;
  |

error[E0412]: cannot find type `Segment` in this scope
 --> src/memory/mapping/memory_set.rs:6:23
  |
6 |     pub segments: Vec<Segment>,
  |                       ^^^^^^^ not found in this scope
  |
help: consider importing this struct
  |
1 | use crate::memory::Segment;
  |

error[E0412]: cannot find type `MemoryResult` in this scope
  --> src/memory/mapping/memory_set.rs:11:28
   |
2  | pub struct MemorySet {
   | -------------------- similarly named struct `MemorySet` defined here
...
11 |     pub fn new_kernel() -> MemoryResult<MemorySet> {
   |                            ^^^^^^^^^^^^
   |
help: a struct with a similar name exists
   |
11 |     pub fn new_kernel() -> MemorySet<MemorySet> {
   |                            ^^^^^^^^^
help: consider importing this type alias
   |
1  | use crate::memory::MemoryResult;
   |

error[E0433]: failed to resolve: use of undeclared type or module `Mapping`
  --> src/memory/mapping/memory_set.rs:53:27
   |
53 |         let mut mapping = Mapping::new()?;
   |                           ^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
1  | use crate::memory::mapping::Mapping;
   |

error[E0433]: failed to resolve: use of undeclared type or module `Vec`
  --> src/memory/mapping/memory_set.rs:55:35
   |
55 |         let mut allocated_pairs = Vec::new();
   |                                   ^^^ not found in this scope
   |
help: consider importing one of these items
   |
1  | use alloc::vec::Vec;
   |
1  | use crate::memory::mapping::mapping::Vec;
   |

error[E0204]: the trait `Copy` may not be implemented for this type
  --> src/memory/mapping/segment.rs:15:10
   |
15 | #[derive(Copy, Clone, Debug, Eq, PartialEq)]
   |          ^^^^
...
18 |     pub map_type: MapType,
   |     --------------------- this field does not implement `Copy`
   |
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors

Some errors have detailed explanations: E0204, E0412, E0433.
For more information about an error, try `rustc --explain E0204`.
error: could not compile `os`.
```

---

Memory_set 中没有实现进程的

```rust
/// 所有分配的物理页面映射信息
    pub allocated_pairs: Vec<(VirtualPageNumber, FrameTracker)>,
```

---

```bash
root@KaDingisoul:~/lab-3-0/os# make run
   Compiling os v0.1.0 (/root/lab-3-0/os)
warning: unused import: `frame::FrameTracker`
 --> src/memory/mapping/memory_set.rs:7:5
  |
7 |     frame::FrameTracker,
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0282]: type annotations needed for `alloc::vec::Vec<T>`
  --> src/memory/mapping/memory_set.rs:69:35
   |
69 |         let mut allocated_pairs = Vec::new();
   |             -------------------   ^^^^^^^^ cannot infer type for type parameter `T`
   |             |
   |             consider giving `allocated_pairs` the explicit type `alloc::vec::Vec<T>`, where the type parameter `T` is specified

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0282`.
error: could not compile `os`.
```

---

os/src/memory/mapping/memory_set.rs 中的 impl MemorySet  new_kernel 映射函数中 ， 

```rust
let mut allocated_pairs = Vec::new();
// 每个字段在页表中进行映射
        for segment in segments.iter() {
            // 同时将新分配的映射关系保存到 allocated_pairs 中
            mapping.map(segment, None)?;
        }
```

因为 map 的定义和我们之后用到的由冲突， 我们还不能将新分配的映射关系保存到allocated_pairs 中，我们将其删掉即可

