use std::thread;
fn main() {
    const CHUNK_SIZE: usize = 10;
    let numbers: Vec<u32> = (1..10000).collect();
    let chunks = numbers.chunks(CHUNK_SIZE);

    let sum = thread::scope(|s| {
        let mut handles = Vec::new();
        for chunk in chunks {
            let h = s.spawn(move || chunk.iter().sum::<u32>());
            handles.push(h);
        }
        handles.into_iter().map(|h| h.join().unwrap()).sum::<u32>()
    });
    println!("The sum is: {:?}", sum);
}
