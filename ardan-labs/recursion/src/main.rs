// use futures::future::BoxFuture;
// use futures::future::FutureExt;

// async fn fibonacci(n: u32) -> BoxFuture<'static, u32> {

use std::pin::Pin;

use async_recursion::*;

#[async_recursion]
async fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

async fn one() {
    println!("One");
}

async fn two() {
    println!("Two");
}

async fn call_one_of_them(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("Invalid choice"),
    }
}

#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", fibonacci(10).await);

    let mut future = async {
        println!("Hello World");
    };

    tokio::pin!(future);
    (&mut future).await;
}
