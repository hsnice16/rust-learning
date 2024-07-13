use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    let handle_2 = thread::spawn(move || {
        println!("Here's a vector: {v:#?}");
    });

    handle.join().unwrap();
    handle_2.join().unwrap();
}
