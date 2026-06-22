#[tokio::main]
async fn main() {
    // 创建一个 broadcast 通道，容量为 16。
    // tx 用于发送消息，rx 是创建通道时自带的第一个接收者。
    let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(16);

    // 创建 20 个新的订阅者。broadcast 的每个订阅者都会收到发送后的消息。
    for n in 0..20 {
        // 每次调用 subscribe 都会基于同一个发送端创建一个新的接收者。
        let mut message = tx.subscribe();

        // 为每个订阅者启动一个异步任务，持续等待并打印收到的消息。
        tokio::spawn(async move {
            while let Ok(msg) = message.recv().await {
                println!("Received: {n} {msg}");
            }
        });
    }

    // 向所有当前订阅者发送一条消息。
    tx.send("Hello, world!".to_string()).unwrap();

    // 使用创建通道时得到的原始接收者接收并打印消息。
    // 因为 tx 仍然存在，这个循环在收到消息后会继续等待后续消息。
    while let Ok(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}
