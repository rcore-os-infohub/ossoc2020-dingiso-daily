lab-4 纠错过程

contex.sepc +=2 出现**无限循环**

当 sepc+= 4 且执行 __restore 的时候

```bash
OpenSBI v0.6
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name          : QEMU Virt Machine
Platform HART Features : RV64ACDFIMSU
Platform Max HARTs     : 8
Current Hart           : 0
Firmware Base          : 0x80000000
Firmware Size          : 120 KB
Runtime SBI Version    : 0.2

MIDELEG : 0x0000000000000222
MEDELEG : 0x000000000000b109
PMP0    : 0x0000000080000000-0x000000008001ffff (A)
PMP1    : 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
mod memory initialized
mod interrupt initialized
thread 1 create successfully!!
thread 1 add successfully!!
thread 2 create successfully!!
thread 2 add successfully!!
thread 3 create successfully!!
thread 3 add successfully!!
thread 4 create successfully!!
thread 4 add successfully!!
thread 5 create successfully!!
thread 5 add successfully!!
thread 6 create successfully!!
thread 6 add successfully!!
thread 7 create successfully!!
thread 7 add successfully!!
thread 8 create successfully!!
thread 8 add successfully!!
hello from kernel thread 7
kernel_thread_exit
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
hello from kernel thread 6
kernel_thread_exit
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
hello from kernel thread 5
kernel_thread_exit
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
hello from kernel thread 4
kernel_thread_exit
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
hello from kernel thread 3
kernel_thread_exit
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
hello from kernel thread 2
kernel_thread_exit
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
hello from kernel thread 1
kernel_thread_exit
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
hello from kernel thread 8
kernel_thread_exit
Breakpoint at 0xffffffff8020122a
Breakpoint at 0xffffffff8020122a
```

猜测 qemu   访问触发breakpoint异常时 可能 ALIGN = 4 bytes ， 所以 4 可以 ， 2 不行 

所以，`context.sepc += 2` 可以忽略不计，只有 4 的倍数才会起作用；只要在错误流程处理的任何一个地方加上这句话或者类似的都能起到效果。