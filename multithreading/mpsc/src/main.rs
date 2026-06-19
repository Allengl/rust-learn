use std::{sync::mpsc, thread};

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Msg {
    Call(Task),
    Quit,
}

fn hello() {
    println!("Hello world!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Msg>();
    let handle = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                Msg::Call(task) => task(),
                Msg::Quit => break,
            }
        }
    });
    let closure = || {
        println!("Hello from Closure!");
    };
    // tx.send(Box::new(hello)).unwrap();
    // tx.send(Box::new(closure)).unwrap();
    // tx.send(Box::new(|| {
    //     println!("Hello from Anonymous Function!");
    // }))
    // .unwrap();
    tx.send(Msg::Call(Box::new(hello))).unwrap();
    tx.send(Msg::Call(Box::new(closure))).unwrap();
    tx.send(Msg::Call(Box::new(|| {
        println!("Hello from Anonymous Function!");
    })))
    .unwrap();
    tx.send(Msg::Quit).unwrap();

    handle.join().unwrap();
}
