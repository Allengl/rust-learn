// AsyncBufReadExt 提供 read_line 等缓冲读取方法
// AsyncWriteExt 提供 write_all 等异步写入方法
// BufReader 用于按行读取 socket 数据
// SocketAddr 作为广播载荷中的发送者标识，用于接收方跳过自己发的消息（消除回声）
use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    signal,
    sync::broadcast,
};
use tokio_util::sync::CancellationToken;

// 将 main 函数变为异步入口点，由 tokio 运行时驱动
#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // 绑定并监听本机 8080 端口，等待客户端连接
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    // 创建广播器，用于向所有连接的客户端广播消息
    // 载荷为 (发送者地址, 消息内容)，便于接收方跳过自己发的消息（消除回声）
    let (tx, _) = broadcast::channel::<(SocketAddr, String)>(10);
    let token = CancellationToken::new();
    let cancel_token = token.clone();
    tokio::spawn(async move {
        tracing::info!("Spawning signal handler...");
        match signal::ctrl_c().await {
            Ok(_) => {
                tracing::info!("Received ctrl-c, shutting down");
                cancel_token.cancel();
            }
            Err(e) => {
                tracing::error!("Failed to listen for ctrl-c: {}", e);
            }
        }
    });
    // 外层循环：不断接收新连接，使服务器能服务多个客户端（而非只处理一次）
    loop {
        let token = token.clone();
        // 克隆广播器，用于每个连接的独立广播
        let tx = tx.clone();
        // 订阅广播器，用于接收广播消息
        let mut rx = tx.subscribe();
        // 用 select! 同时等待「新连接」和「取消信号」
        // 这样即使没有客户端连接，服务器在 accept 上也能响应 Ctrl+C 退出
        let (mut socket, address) = tokio::select! {
            // 收到取消信号（Ctrl+C），跳出主循环，让 main 返回
            _ = token.cancelled() => {
                tracing::info!("主循环收到取消信号，准备关闭服务器");
                break;
            }
            // 有新客户端连接到来，解包出 (socket, address)
            result = listener.accept() => result.unwrap(),
        };

        // 为每个连接 spawn 一个独立的 tokio 任务，实现并发处理
        // async move 把 socket 的所有权移动进任务，避免跨任务借用
        tokio::spawn(async move {
            let (steam_reader, mut steam_writer) = socket.split();
            let mut message = String::new();
            // 用 BufReader 包装读半部，从而可以使用按行读取
            let mut reader = BufReader::new(steam_reader);
            // 回显（echo）循环：每读到一行就原样写回客户端
            loop {
                tokio::select! {
                    // pattern = future => handler
                    result = reader.read_line(&mut message) => {
                        tracing::info!("Read line: {}", message.trim());
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((address, message.clone())).unwrap();
                        message.clear();
                    }
                    result = rx.recv() => {
                        let (sender_addr, received_message) = result.unwrap();
                        // 跳过自己发的消息，避免回声
                        if sender_addr != address {
                            tracing::info!("Received message: {}", received_message.trim());
                            steam_writer
                                .write_all(received_message.as_bytes())
                                .await
                                .unwrap();
                        }
                    }
                    _ = token.cancelled() => {
                        tracing::info!("Token cancelled, cleaning up");
                        println!("cleaning up");
                        return;
                    }
                };
            }
        });
    }
}
