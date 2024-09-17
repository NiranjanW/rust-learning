// https://medium.com/@md.abir1203/async-and-the-future-how-it-changes-the-coding-style-in-rust-a2a152e6c86d

use std::future::Future;
use std::time::{Duration , Instant};
use tokio::time::sleep;

fn slow_io_operation( id :u32) ->String{
    std::thread::sleep(Duration::from_secs((2)));
    format!("Result from operation {}", id)
}

fn run_sync_operations() {
    let start = Instant::now();
    let result1 = slow_io_operation(1);
    println!("Sync: {}", result1);
    
    let result2 = slow_io_operation(2);
    println!("Sync: {}", result2);
    
    let result3 = slow_io_operation(3);
    println!("Sync: {}", result3);
    
    println!("Sync total time: {:?}", start.elapsed());
}

async fn add_async(a: i32 , b :i32) -> i32 {
    sleep(Duration::from_millis(2000)).await;
    a + b
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let result1 = add_async(2, 3).await;
    let result2 = add_async(4, 5).await;
    let elapsed = start.elapsed();
    println!("Async results: {} and {}", result1, result2);
    println!("Async total time: {:?}", elapsed);
}

// fn main() {
//     println!("Hello, world!");
//     let x = foo1();
// }

fn foo1() -> usize {
    0
}

async fn foo2() -> impl Future<Output = usize>{
    async{0}
}
