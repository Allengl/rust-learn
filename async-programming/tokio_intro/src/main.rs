async fn hello(task: u64, time: u64) {
    println!(
        "Task {task} started on thread {:?}",
        std::thread::current().id()
    );
    tokio::time::sleep(std::time::Duration::from_millis(time)).await;
    println!("Task {task} finished.");
}

#[tokio::main]
async fn main() {
    tokio::join!(hello(1, 100), hello(2, 200), hello(3, 300));
}
