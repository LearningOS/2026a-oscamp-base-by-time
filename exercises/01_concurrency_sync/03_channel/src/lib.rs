//! # 通道通信
//!
//! 在本练习中，你将使用 `std::sync::mpsc` 通道在线程之间传递消息。
//!
//! ## 概念
//! - `mpsc::channel()` 创建一个多生产者、单消费者通道
//! - `Sender::send()` 发送一条消息
//! - `Receiver::recv()` 接收一条消息
//! - 可以通过 `Sender::clone()` 创建多个生产者

use std::sync::mpsc;
use std::thread;
/// 创建一个生产者线程，把 items 中的每个元素发送到通道中。
/// 主线程接收所有消息并把它们返回。
pub fn simple_send_recv(items: Vec<String>) -> Vec<String> {
    // TODO: 创建通道
    // TODO: 启动线程，发送 items 中的每个元素
    // TODO: 在主线程中接收所有消息并收集到 Vec
    // 提示：当所有 Sender 都被丢弃后，recv() 会返回 Err
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in items.into_iter() {
            tx.send(i).unwrap();
        }
    });

    let mut  a: Vec<String> = Vec::new();

    loop {
        match rx.recv() {
            Ok(x) => a.push(x),
            Err(_) => break,
        }
    };

    a

}

/// 创建 `n_producers` 个生产者线程，每个线程发送一条格式为 `"msg from {id}"` 的消息。
/// 收集所有消息，按字典序排序后返回。
///
/// 提示：使用 `tx.clone()` 创建多个发送者。注意原始的 tx 也必须被丢弃。
pub fn multi_producer(n_producers: usize) -> Vec<String> {
    // TODO: 创建通道
    // TODO: 为每个生产者克隆一个发送者
    // TODO: 记得丢弃原始发送者，否则接收方不会结束
    // TODO: 收集所有消息并排序
    let (tx, rx) = mpsc::channel();

    for i in 0..n_producers {
        let a = tx.clone();
        let c= thread::spawn(move || {
            a.send(format!("msg from {i}")).unwrap();
        });

        c.join().unwrap();
    };

    drop(tx);

    let mut b: Vec<String> = Vec::new();
    for received in rx {
        b.push(received.to_string())
    };
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_send_recv() {
        let items = vec!["hello".into(), "world".into(), "rust".into()];
        let result = simple_send_recv(items.clone());
        assert_eq!(result, items);
    }

    #[test]
    fn test_simple_empty() {
        let result = simple_send_recv(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multi_producer() {
        let result = multi_producer(3);
        assert_eq!(
            result,
            vec![
                "msg from 0".to_string(),
                "msg from 1".to_string(),
                "msg from 2".to_string(),
            ]
        );
    }

    #[test]
    fn test_multi_producer_single() {
        let result = multi_producer(1);
        assert_eq!(result, vec!["msg from 0".to_string()]);
    }
}
