// // use tokio::runtime;

async fn hello() -> u32 {
    println!("Hello Tokio");
    3
}

// // fn main() {
// //     let rt = runtime::Builder::new_multi_thread()
// //         .enable_all()
// //         .worker_threads(4)
// //         .build()
// //         .unwrap();

// //     rt.block_on(hello());
// // }

// async fn hello2() -> u32 {
//     println!("Hello Tokio2");
//     4
// }

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // hello().await;

    // let result = tokio::join!(hello(), hello2());
    // println!("{result:?}");

    // let (one, two) = result;
    // println!("one: {one}, two: {two}");

    // tokio::spawn(ticker());
    // // hello().await;

    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker())
    );
    println!("Finished");
}
