use std::{sync::Arc, thread};

fn main() {
    let data = Arc::new([1, 2, 3, 4, 5]);
    let mut handles = Vec::new();
    for _ in 0..10000 {
        let local_data = data.clone();
        let h = thread::spawn(move || {
            println!("Data: {local_data:?}");
        });
        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}
