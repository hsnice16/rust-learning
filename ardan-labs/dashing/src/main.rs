use std::time::Duration;

use dashmap::DashMap;
use once_cell::sync::Lazy;

static SHARED: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);

fn main() {
    for n in 0..100 {
        std::thread::spawn(move || {
            loop {
                if let Some(mut v) = SHARED.get_mut(&n) {
                    *v += 1;
                } else {
                    SHARED.insert(n, n);
                }
            }
        });
    }

    std::thread::sleep(Duration::from_secs(2));
    println!("{SHARED:#?}");
}
