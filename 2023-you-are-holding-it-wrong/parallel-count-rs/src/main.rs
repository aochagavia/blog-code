use std::thread;
use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    let counter = AtomicU32::new(0);

    thread::scope(|s| {
        let t1 = s.spawn(|| {
            for _ in 0..50000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });

        let t2 = s.spawn(|| {
            for _ in 0..50000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();

        println!("Result = {}", counter.load(Ordering::Relaxed));
    });

}
