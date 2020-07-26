use super::*;
use algorithm::*;
use hashbrown::HashSet;
use lazy_static::*;

lazy_static! {
    /// 全局的 [`Processor`]
    //pub static ref PROCESSOR: UnsafeWrapper<Processor> = Default::default();
    pub static ref PROCESSOR: Lock<Processor> = Lock::new(Processor::default());
}

lazy_static! {
    /// 空闲线程：当所有线程进入休眠时，切换到这个线程——它什么都不做，只会等待下一次中断
    static ref IDLE_THREAD: Arc<Thread> = Thread::new(
        Process::new_kernel().unwrap(),
        wait_for_interrupt as usize,
        None,
    ).unwrap();
}

/// 不断让 CPU 进入休眠等待下一次中断
unsafe fn wait_for_interrupt() {
    loop {
        llvm_asm!("wfi" :::: "volatile");
    }
}
/// 线程调度和管理
#[derive(Default)]
pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程
    sleeping_threads: HashSet<Arc<Thread>>,
}

impl Processor {
    /// 获取一个当前线程的 `Arc` 引用
    pub fn current_thread(&self) -> Arc<Thread> {
        self.current_thread.as_ref().unwrap().clone()
    }
    /// 在一个时钟中断时，替换掉 context
    pub fn tick(&mut self, context: &mut Context) -> *mut Context {
        // 向调度器询问下一个线程
        if let Some(next_thread) = self.scheduler.get_next() {
            if next_thread == self.current_thread() {
                // 没有更换线程，直接返回 Context
                context
            } else {
                // 准备下一个线程
                let next_context = next_thread.run();
                let current_thread = self.current_thread.replace(next_thread).unwrap();
                // 储存当前线程 Context
                current_thread.park(*context);
                // 返回下一个线程的 Context
                next_context
            }
        } else {
            panic!("all threads terminated, shutting down");
        }
    }

    /// 第一次开始运行
    ///
    /// 从 `current_thread` 中取出 [`Context`]，然后直接调用 `interrupt.asm` 中的 `__restore`
    /// 来从 `Context` 中继续执行该线程。
    ///
    /// 注意调用 `run()` 的线程会就此步入虚无，不再被使用
    pub fn run(&mut self) -> ! {
        // interrupt.asm 中的标签
        extern "C" {
            fn __restore(context: usize);
        }
        // 从 current_thread 中取出 Context
        if self.current_thread.is_none() {
            panic!("no thread to run, shutting down");
        }
        let context = self.current_thread().prepare();
        // 从此将没有回头
        unsafe {
            __restore(context as usize);
        }
        unreachable!()
    }

    /// 保存当前线程的 `Context`
    pub fn park_current_thread(&mut self, context: &Context) {
        self.current_thread().park(*context);
    }
    
     /// 激活下一个线程的 `Context`
     pub fn prepare_next_thread(&mut self) -> *mut Context {
        // 向调度器询问下一个线程
        if let Some(next_thread) = self.scheduler.get_next() {
            // 准备下一个线程
            let context = next_thread.prepare();
            self.current_thread = Some(next_thread);
            context
        } else {
            // 没有活跃线程
            if self.sleeping_threads.is_empty() {
                // 也没有休眠线程，则退出
                panic!("all threads terminated, shutting down");
            } else {
                // 有休眠线程，则等待中断
                self.current_thread = Some(IDLE_THREAD.clone());
                IDLE_THREAD.prepare()
            }
        }
    }

    /// 添加一个待执行的线程
    pub fn add_thread(&mut self, thread: Arc<Thread>) {
        if self.current_thread.is_none() {
            self.current_thread = Some(thread.clone());
        }
        self.scheduler.add_thread(thread, 0);
    }
}
