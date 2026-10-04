//! # 互斥锁共享状态
//!
//! 在本练习中，你将使用 `Arc<Mutex<T>>` 在多个线程之间安全地共享和修改数据。
//!
//! ## 概念
//! - `Mutex<T>` 互斥锁保护共享数据
//! - `Arc<T>` 原子引用计数实现跨线程共享
//! - `lock()` 获取锁并访问数据

use std::sync::{Arc, Mutex};
use std::thread;

/// 使用 `n_threads` 个线程并发地对计数器进行递增。
/// 每个线程将计数器递增 `count_per_thread` 次。
/// 返回计数器的最终值。
///
/// 提示：使用 `Arc<Mutex<usize>>` 作为共享计数器。
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // TODO: 创建初始值为 0 的 Arc<Mutex<usize>>
    // TODO: 启动 n_threads 个线程
    // TODO: 在每个线程中调用 lock() 并递增 count_per_thread 次
    // TODO: join 所有线程，返回最终值

    let a = Arc::new(Mutex::new(0));
    for _i in 0 .. n_threads {
        thread::scope(|s| {
            let _h1 = s.spawn(|| {
                let b = Arc::clone(&a);
                let mut c = b.lock().unwrap();
                for _i in 0..count_per_thread {
                    *c += 1;
                }
            });
        })
    }
    *a.clone().lock().unwrap()
}

/// 使用多个线程并发地向共享向量添加元素。
/// 每个线程将自己的 id（0..n_threads）压入该向量。
/// 返回排序后的向量。
///
/// 提示：使用 `Arc<Mutex<Vec<usize>>>`。
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: 创建 Arc<Mutex<Vec<usize>>>
    // TODO: 每个线程压入自己的 id
    // TODO: join 所有线程后，对结果排序并返回
    let a = Arc::new(Mutex::new(Vec::new()));
    for id in 0..n_threads {
        // thread::scope(|s| {
        //     let _handler = s.spawn(|| {
        //         let b = Arc::clone(&a);
        //         let mut c = b.lock().unwrap();
        //         c.push(id);
        //     });
        // })
        let b = Arc::clone(&a);
        let e =  thread::spawn(move || {
            let mut c = b.lock().unwrap();
            c.push(id);
        });
        e.join().unwrap();
    }
    let d = Arc::clone(&a);
    let x = d.lock().unwrap().clone();x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
