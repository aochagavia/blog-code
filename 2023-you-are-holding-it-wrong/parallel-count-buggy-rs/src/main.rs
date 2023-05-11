use std::thread;

fn main() {
    let mut counter = 0;

    thread::scope(|s| {
        let t1 = s.spawn(|| {
            for _ in 0..50000 {
                counter += 1;
            }
        });

        let t2 = s.spawn(|| {
            for _ in 0..50000 {
                counter += 1;
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();

        println!("Result = {counter}");
    });

}
