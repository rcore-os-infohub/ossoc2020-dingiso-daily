# lab-2

```bash
error[E0433]: failed to resolve: use of undeclared type or module `memory`
  --> src/main.rs:40:5
   |
40 |     memory::init();
   |     ^^^^^^ use of undeclared type or module `memory`

```

写 src/memory/mod.rs 

```rust
mod config;
mod heap;

pub fn init() {
    heap::init();
    println!("mod memory initialized");
}
```

---

找不到alloc 的错误

```rust
extern crate alloc;
//为什么就他需要 core 都不需要
```

---

```bash
error[E0658]: the `#[alloc_error_handler]` attribute is an experimental feature
  --> src/memory/heap.rs:32:1
   |
32 | #[alloc_error_handler])-
   | ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #51540 <https://github.com/rust-lang/rust/issues/51540> for more information
   = help: add `#![feature(alloc_error_handler)]` to the crate attributes to enable

```

main.rs 添加

---

```bash
error[E0432]: unresolved imports `super::config::KERNEL_MAP_OFFSET`, `super::config::PAGE_SIZE`
 --> src/memory/address.rs:5:21
  |
5 | use super::config::{KERNEL_MAP_OFFSET, PAGE_SIZE};
  |                     ^^^^^^^^^^^^^^^^^  ^^^^^^^^^ no `PAGE_SIZE` in `memory::config`
  |                     |
  |                     no `KERNEL_MAP_OFFSET` in `memory::config`
	# config 文件更改
error[E0425]: cannot find value `KERNEL_END_ADDRESS` in `memory::config`
  --> src/main.rs:44:37
   |
44 |     println!("{}", *memory::config::KERNEL_END_ADDRESS);
   |                                     ^^^^^^^^^^^^^^^^^^ not found in `memory::config`

error[E0603]: module `config` is private
  --> src/main.rs:44:29
   |
44 |     println!("{}", *memory::config::KERNEL_END_ADDRESS);
   |                             ^^^^^^ private module
   |
note: the module `config` is defined here
  --> src/memory/mod.rs:2:1
   |
2  | mod config;
   | ^^^^^^^^^^^
# mod.rs pub mod config;
error: aborting due to 3 previous errors

```

---

```bash
error[E0433]: failed to resolve: could not find `frame` in `memory`
  --> src/main.rs:46:37
   |
46 |         let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
   |                                     ^^^^^ could not find `frame` in `memory`                                                                       

error[E0433]: failed to resolve: could not find `frame` in `memory`
  --> src/main.rs:50:37
   |
50 |         let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
   |                                     ^^^^^ could not find `frame` in `memory`                                                                       

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0433`.
error: could not compile `os`.


```

---

![批注 2020-07-23 153014](C:\Users\luruibo2000\Pictures\批注 2020-07-23 153014.png)

```rust
lazy_static! {
    /// 内核代码结束的地址，即可以用来分配的内存起始地址
    ///
    /// 因为 Rust 语言限制，我们只能将其作为一个运行时求值的 static 变量，而不能作为 const
//    pub static ref KERNEL_END_ADDRESS: VirtualAddress = //VirtualAddress(kernel_end as usize); 更改前
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}
```

KERNEL_END_ADDRESS 直接利用了 lab-3 的 忘记进行更改

---

### 结果：

![lab-2](C:\Users\luruibo2000\Pictures\lab-2.png)