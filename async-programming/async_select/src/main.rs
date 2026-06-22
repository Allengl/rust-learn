use std::time::Duration;

use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};

// 同时监听一个 mpsc 接收端和一个 broadcast 接收端。
// `select!` 会等待多个异步操作中最先完成的那个分支，并执行对应逻辑。
async fn receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            // mpsc 是多生产者、单消费者通道；这里只要收到一个数字就打印。
            Some(n) = rx.recv() => println!("从 mpsc channel 收到消息 {n}"),
            // broadcast 是广播通道；每个订阅者都会收到一份发送出去的消息。
            Ok(n) = broadcast_rx.recv() => println!("从 broadcast channel 收到消息 {n}"),
        }
    }
}

#[tokio::main]
async fn main() {
    // 创建容量为 1 的 mpsc channel。tx 用于发送，rx 用于接收。
    let (tx, rx) = mpsc::channel::<u32>(1);
    // 创建容量为 1 的 broadcast channel。发送端发送一次，所有订阅者都能接收。
    let (broadcast_tx, broadcast_rx) = broadcast::channel::<u32>(1);

    // 启动一个异步任务，让它同时接收两个 channel 里的消息。
    tokio::spawn(receiver(rx, broadcast_rx));

    for c in 0..10 {
        if c % 2 == 0 {
            // 偶数通过 mpsc channel 发送。
            tx.send(c).await.unwrap();
        } else {
            // 奇数通过 broadcast channel 发送。
            broadcast_tx.send(c).unwrap();
        }
        // 每秒发送一次，方便观察两个 channel 的输出顺序。
        sleep(Duration::from_secs(1)).await;
    }
}
