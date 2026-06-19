use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

fn main() {
    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..1000 {
            s.spawn(|| {
                incr(&counter);
            });
        }

        // loop {
        //     let n = done.load(Ordering::Relaxed);
        //     if n == 1000 {
        //         break;
        //     }
        //     println!("Progress: {n}/1000 done");
        //     thread::sleep(Duration::from_secs(1));
        // }
    });

    println!("Counter done! {}", { counter.load(Ordering::Relaxed) });
}

fn incr(a: &AtomicUsize) {
    let mut current = a.load(Ordering::Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(v) => {
                println!("Failed to update: expected {current}, got {new}");
                current = v;
            }
        }
    }
}
