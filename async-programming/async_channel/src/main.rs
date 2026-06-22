use std::{sync::mpsc, thread, time::Duration};

#[tokio::main]
async fn main() {
    // 标准库的 mpsc 通道，用来把任务从 Tokio 异步上下文发送到普通 OS 线程。
    let (tx, rx) = mpsc::channel::<Task>();

    // Tokio 的异步 mpsc 通道，用来把普通线程中的计算结果发送回异步任务。
    // 缓冲区大小为 20，表示最多可以暂存 20 个还没有被接收的结果。
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<i32>(20);

    // 保存当前 Tokio runtime 的句柄。
    // 普通线程不能直接 `.await`，但可以通过这个句柄把 async 任务提交回 Tokio runtime 执行。
    let handle = tokio::runtime::Handle::current();

    // 启动一个普通线程，专门接收并处理同步 mpsc 通道中的任务。
    thread::spawn(move || {
        // `recv()` 是阻塞调用：没有任务时线程会停在这里等待。
        // 当所有发送端都被丢弃后，`recv()` 返回 Err，循环结束。
        while let Ok(task) = rx.recv() {
            match task {
                Task::Calculate(n) => {
                    // clone 一个发送端给下面的 async 任务使用。
                    let tx_reply = tx_reply.clone();
                    let result = n * n;

                    // 把发送结果的动作交回 Tokio runtime。
                    // 这里不能直接 `.await`，因为当前代码运行在普通线程里，不在 async 函数中。
                    handle.spawn(async move {
                        tx_reply.send(result).await.unwrap();
                    });
                }
            }
        }
    });

    // 启动一个 Tokio 异步任务，持续接收计算结果并打印。
    tokio::spawn(async move {
        // `recv().await` 是异步等待：没有结果时不会阻塞整个 runtime 线程。
        // 当所有 tx_reply 发送端都被丢弃后，返回 None，循环结束。
        while let Some(result) = rx_reply.recv().await {
            println!("Result: {}", result);
        }
    });

    let mut num = 1;
    loop {
        // 每 500ms 生成一个新任务。
        tokio::time::sleep(Duration::from_millis(500)).await;

        // 把计算任务发送给普通线程处理。
        tx.send(Task::Calculate(num)).unwrap();
        num += 1;
    }
}

// 当前任务类型只有一种：计算一个整数的平方。
enum Task {
    Calculate(i32),
}
