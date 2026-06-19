use std::thread;

fn main() {
    let handle = thread::Builder::new()
        .name("Thread 1".into())
        .stack_size(4 * 1024 * 1024) // 4 MB
        .spawn(another_thread)
        .unwrap();
    handle.join().unwrap();
}

fn another_thread() {
    println!("In thread: {}", thread::current().name().unwrap());
}
