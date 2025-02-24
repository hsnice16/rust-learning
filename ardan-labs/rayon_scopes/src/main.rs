fn test() {
    println!("test");
}

fn main() {
    // Explicitly sized pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    // // pool.spawn(|| println!("Hello from pool thread"));

    // // pool.scope(|scope| {
    // //     for n in 0..20 {
    // //         scope.spawn(move |_| println!("Hello from scoped thread {n}"));
    // //     }
    // // });

    // // println!("Hello from the main thread");

    // pool.scope(|scope| {
    //     scope.spawn_broadcast(|_scope, braodcast_context| {
    //         println!("Hello from broadcast thread {}", braodcast_context.index());
    //     });
    // });

    pool.join(test, test);
}
