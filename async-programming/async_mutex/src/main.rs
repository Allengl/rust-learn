use std::{sync::LazyLock, time::Duration};
use tokio::{sync::Mutex, time::sleep};

static DATA: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

async fn task1() {
    println!("Task1 try lock");
    let _guard = DATA.lock().await;
    println!("Task1 locked, sleep 5s");
    sleep(Duration::from_secs(5)).await;
    println!("Task1 done");
}

async fn task2() {
    sleep(Duration::from_millis(100)).await;
    println!("Task2 try lock");
    let _guard = DATA.lock().await;
    println!("Task2 locked");
}

#[tokio::main]
async fn main() {
    tokio::join!(task1(), task2());
}
