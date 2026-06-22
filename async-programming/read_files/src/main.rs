use std::{
    fs::File as StdFile,
    io::{self, BufRead},
    path::Path,
    time::Instant,
};

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

// #[tokio::main] 会创建 Tokio 运行时，让 main 函数可以直接使用 .await。
#[tokio::main]
async fn main() {
    // 先执行同步版本，作为和异步版本对比的基准。
    let now = Instant::now();
    let line_count = count_lines_sync();
    println!(
        "Sync version: read {} lines in {:.3} seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );

    // join! 会在同一个异步任务中并发轮询两个 future。
    // ticker 能否持续输出，可以用来观察异步读文件是否把运行时线程长时间阻塞住。
    let _ = tokio::join!(count_lines_async("src/main.rs"), ticker());
}

fn count_lines_sync() -> i32 {
    println!("start executing sync count line");
    let mut count = 0;
    if let Ok(lines) = read_lines("src/main.rs") {
        lines.for_each(|line| {
            if let Ok(line) = line
                && !line.trim().is_empty()
            {
                count += 1;
            }
        });
    }
    count
}

// 同步版本：使用 std::fs::File 打开文件，读取过程会阻塞当前线程。
fn read_lines<T>(filename: T) -> anyhow::Result<io::Lines<io::BufReader<StdFile>>>
where
    T: AsRef<Path>,
{
    let file = StdFile::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines())
}

async fn count_lines_async<T>(filename: T) -> anyhow::Result<usize>
where
    T: AsRef<Path>,
{
    let now = std::time::Instant::now();
    let mut count = 0;

    if let Ok(mut lines) = read_lines_async(filename).await {
        // next_line().await 每次异步读取一行；等待 I/O 时会把执行权交还给 Tokio 运行时。
        while let Some(line) = lines.next_line().await? {
            if !line.trim().is_empty() {
                count += 1;
            }
        }
    }

    println!(
        "Async version: read {} lines in {:.3} seconds",
        count,
        now.elapsed().as_secs_f32()
    );

    Ok(count)
}

// 异步版本：使用 tokio::fs::File 打开文件，配合 Tokio 的 BufReader 和 lines。
async fn read_lines_async<T>(filename: T) -> anyhow::Result<tokio::io::Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

// 模拟另一个异步任务，每隔 5ms 输出一次 tick。
async fn ticker() {
    for _ in 0..10 {
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        println!("tick");
    }
}
