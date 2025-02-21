// static mut COUNTER: i32 = 0;

use std::sync::atomic::AtomicI32;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut thread_handles = Vec::new();

    // 1000 threads
    for _ in 0..1000 {
        let thread_handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
