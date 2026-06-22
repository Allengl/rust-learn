use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    // 根据数据源选择一个异步任务。这里返回的是被 Pin 固定住的堆分配 Future。
    let task = get_task("db");

    // 等待这个动态分发的 Future 执行完成，拿到最终的 String 结果。
    let result = task.await;
    println!("Got result: {result}");
}

// 不同的 async fn 会生成不同的匿名 Future 类型。
// 为了在 match 的不同分支中返回同一种类型，需要把 Future 装进 Box，
// 再用 Pin 固定它的位置，最终统一成 Pin<Box<dyn Future<Output = String>>>。
fn get_task(source: &str) -> Pin<Box<dyn Future<Output = String>>> {
    match source {
        // Box::pin 会把 Future 放到堆上，并返回 Pin<Box<T>>。
        "db" => Box::pin(from_db()),
        "api" => Box::pin(from_api()),
        _ => Box::pin(async { "Unknown source".to_string() }),
    }
}

// 模拟从数据库中异步获取数据。
async fn from_db() -> String {
    "DB data".to_string()
}

// 模拟从 API 中异步获取数据。
async fn from_api() -> String {
    "API data".to_string()
}
