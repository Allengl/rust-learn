use futures::{executor, join};

async fn hi() {
    println!("Hi!");
}

async fn hello() {
    println!("Hello World!");
    hello_sync();
}

fn hello_sync() {
    println!("Hello sync!");
}

async fn do_mul() {
    join!(hi(), hello());
    let sum = add(1, 2).await;
    println!("sum: {}", sum);

    let (a, b) = join!(add(1, 2), add(3, 4));
    println!("Sums: a = {}, b = {}", a, b);
}

async fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // 1. async fn 可以执行 non-async fn
    // 2. non-async fn 不可以执行 async fn，除非有 executor
    let func = do_mul(); //executor
    executor::block_on(func);
}
