```bash
error[E0433]: failed to resolve: use of undeclared type or module `Trap`
  --> src/interrupt/handler.rs:31:9
   |
31 |         Trap::Exception(Exception::Breakpoint) => breakpoint(context),
   |         ^^^^ use of undeclared type or module `Trap`

error[E0433]: failed to resolve: use of undeclared type or module `Exception`
  --> src/interrupt/handler.rs:31:25
   |
31 |         Trap::Exception(Exception::Breakpoint) => breakpoint(context),
   |                         ^^^^^^^^^ use of undeclared type or module `Exception`

error[E0433]: failed to resolve: use of undeclared type or module `Trap`
  --> src/interrupt/handler.rs:33:9
   |
33 |         Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
   |         ^^^^ use of undeclared type or module `Trap`

error[E0433]: failed to resolve: use of undeclared type or module `Interrupt`
  --> src/interrupt/handler.rs:33:25
   |
33 |         Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
   |                         ^^^^^^^^^ use of undeclared type or module `Interrupt`

error[E0433]: failed to resolve: use of undeclared type or module `timer`
  --> src/interrupt/handler.rs:51:5
   |
51 |     timer::tick();
   |     ^^^^^ use of undeclared type or module `timer`
   
   # 前面的错误都是关于use 的问题 ：use super::timer;
   #use riscv::register::scause::{Exception, Interrupt, Scause, Trap};

error[E0425]: cannot find function `set_next_timeout` in this scope
  --> src/interrupt/timer.rs:17:5
   |
17 |     set_next_timeout();
   |     ^^^^^^^^^^^^^^^^ not found in this scope
   # 调整顺序

warning: unused import: `crate::sbi::set_timer`
 --> src/interrupt/timer.rs:3:5
  |                                                                                     
3 | use crate::sbi::set_timer;                                                          
  |     ^^^^^^^^^^^^^^^^^^^^^                                                           
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `time`
 --> src/interrupt/timer.rs:4:37
  |
4 | use riscv::register::{sie, sstatus, time};
  |                                     ^^^^

error[E0308]: mismatched types
  --> src/main.rs:37:34
   |
37 | pub extern "C" fn rust_main() -> ! {
   |                   ---------      ^ expected `!`, found `()`
   |                   |
   |                   implicitly returns `()` as its body has no tail or `return` expression                                                                                   
   |
   = note:   expected type `!`
           found unit type `()`
	#  返回 ()
error[E0277]: `interrupt::context::Context` doesn't implement `core::fmt::Debug`
  --> src/interrupt/handler.rs:59:9
   |
59 |         context,
   |         ^^^^^^^ `interrupt::context::Context` cannot be formatted using `{:?}`
   |
   = help: the trait `core::fmt::Debug` is not implemented for `interrupt::context::Context`
   = note: add `#[derive(Debug)]` or manually implement `core::fmt::Debug`
   = note: required because of the requirements on the impl of `core::fmt::Debug` for `&mut interrupt::context::Context`
   = note: required by `core::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   # context.rs Context添加 #[derive(Clone, Copy, Debug)]
error: aborting due to 8 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0277, E0308, E0425, E0433.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `os`.

To learn more, run the command again with --verbose.
make: *** [Makefile:20：kernel] 错误 101
```

