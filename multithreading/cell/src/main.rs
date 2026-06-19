// use std::{
//     cell::{Cell, OnceCell, RefCell},
//     sync::{
//         Arc, Mutex, OnceLock, RwLock,
//         atomic::{AtomicU32, Ordering},
//     },
//     thread,
// };

// static LIST: OnceList<u32> = OnceList::new();
// static COUNTER: AtomicU32 = AtomicU32::new(0);

// const LEN: u32 = 1000;

// fn main() {
//     // Cell<T>
//     // let cell = Cell::new(5);
//     // assert_eq!(cell.get(), 5);
//     // assert_eq!(cell.replace(10), 5);
//     // assert_eq!(cell.get(), 10);

//     // let ten = cell.into_inner();
//     // assert_eq!(ten, 10);

//     // let cell = Cell::new(String::from("Hello"));
//     // assert_eq!(cell.take(), "Hello");
//     // assert_eq!(cell.take(), String::default()); // After take, the cell is empty

//     // cell.set(String::from("World"));
//     // assert_eq!(cell.take(), "World");
//     //

//     // RefCell<T>
//     // let rc = RefCell::new(5);
//     // {
//     //     let five = rc.borrow();
//     //     let five1 = rc.borrow();
//     // }
//     // let mut f = rc.borrow_mut();
//     // *f += 6;
//     // // drop(f);

//     // // *rc.borrow_mut() += 1;
//     // let v = rc.try_borrow();
//     // assert!(v.is_err());

//     // println!("{rc:#?}");
//     //
//     // Mutex
//     // let handles = Vec::new();
//     // for _ in 0..20 {
//     //     let h = thread::spawn(|| {
//     //         let mut lock = NUMBERS.lock().unwrap();
//     //         lock.push(1);
//     //     });
//     //     handles.push(h);
//     // }
//     // handles.into_iter().for_each(|h| h.join().unwrap());

//     // let lock = NUMBERS.lock().unwrap();
//     // println!("{lock:#?}");
//     //
//     // let data = Arc::new(Mutex::new(0));
//     // {
//     //     let data = Arc::clone(&data);
//     //     thread::spawn(move || {
//     //         let mut lock = data.lock().unwrap();
//     //         *lock += 1;
//     //         panic!();
//     //     })
//     //     .join()
//     //     .unwrap_err();
//     // }

//     // {
//     //     let data = Arc::clone(&data);
//     //     thread::spawn(move || match data.lock() {
//     //         Ok(mut guard) => {
//     //             println!("Thread 2: Acquired lock, value = {}", *guard);
//     //             *guard += 10000;
//     //         }
//     //         Err(poisoned) => {
//     //             let mut guard = poisoned.into_inner();
//     //             *guard += 1;
//     //             println!(
//     //                 "Thread 2: Lock was poisoned, but we can still access the data. value = {}",
//     //                 *guard
//     //             );
//     //         }
//     //     })
//     //     .join()
//     //     .unwrap();
//     // }
//     //Rwlock
//     // let counter = Arc::new(RwLock::new(0));

//     // let mut handles = Vec::new();
//     // for i in 0..10 {
//     //     let counter = Arc::clone(&counter);
//     //     let h = thread::spawn(move || {
//     //         let value = counter.read().unwrap();
//     //         println!("Thread {i}, counter value: {value}");
//     //     });
//     //     handles.push(h);
//     // }
//     // {
//     //     let counter = Arc::clone(&counter);
//     //     let h = thread::spawn(move || {
//     //         let mut value = counter.write().unwrap();
//     //         *value += 1;
//     //         println!("Writer thread incremented counter to: {value}");
//     //     });
//     //     handles.push(h);
//     // }

//     // handles.into_iter().for_each(|h| h.join().unwrap());
//     // println!("Final counter value: {}", counter.read().unwrap());
//     //
//     // OnceCell
//     // let cell = OnceCell::new();
//     // assert!(cell.get().is_none());

//     // let value = cell.get_or_init(|| {
//     //     println!("Initializing value...");
//     //     "Hello, World".to_string()
//     // });

//     // assert_eq!(value, "Hello, World");
//     // assert!(cell.get().is_some())
//     //
//     // let mut cell = OnceCell::new();
//     // let _ = cell.set(String::from("Hello"));

//     // if let Some(value_ref) = cell.get_mut() {
//     //     value_ref.push('!');
//     // }
//     // let _ = cell.set(String::from("World"));

//     // if let Some(value) = cell.get() {
//     //     println!("Value in OnceCell: {value}");
//     // }
//     //
//     //
//     // OnceLock
//     // assert!(LOCK.get().is_none());

//     // thread::spawn(|| {
//     //     let value = LOCK.get_or_init(|| 12345);
//     //     assert_eq!(value, &12345)
//     // })
//     // .join()
//     // .unwrap();

//     // assert_eq!(LOCK.get(), Some(&12345));

//     thread::scope(|s| {
//         for _ in 0..thread::available_parallelism().unwrap().get() {
//             s.spawn(|| {
//                 while let i @ 0..LEN = COUNTER.fetch_add(1, Ordering::Relaxed) {
//                     LIST.push(i);
//                 }
//             });
//         }
//     });
//     for i in 0..LEN {
//         assert!(LIST.contains(&i))
//     }
// }

// struct OnceList<T> {
//     data: OnceLock<T>,
//     next: OnceLock<Box<OnceList<T>>>,
// }

// impl<T> OnceList<T> {
//     const fn new() -> OnceList<T> {
//         OnceList {
//             data: OnceLock::new(),
//             next: OnceLock::new(),
//         }
//     }

//     fn push(&self, value: T) {
//         if let Err(value) = self.data.set(value) {
//             let next = self.next.get_or_init(|| Box::new(OnceList::new()));
//             next.push(value);
//             return;
//         }
//     }

//     fn contains(&self, value: &T) -> bool
//     where
//         T: PartialEq,
//     {
//         self.data
//             .get()
//             .map(|iter| iter == value)
//             .filter(|v| *v)
//             .unwrap_or_else(|| {
//                 self.next
//                     .get()
//                     .map(|next| next.contains(value))
//                     .unwrap_or(false)
//             })
//     }
// }

use std::{sync::LazyLock, thread};

static NUMBER: LazyLock<i32> = LazyLock::new(|| {
    println!("Init");
    100
});

fn main() {
    let handles: Vec<_> = (0..5)
        .map(|_| {
            thread::spawn(|| {
                println!("Thread sees NUMBER = {}", *NUMBER);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
