use std::{thread, time::Duration};

use tokio::task;

#[tokio::main(flavor="multi_thread", worker_threads= 4)] //4개의 worker thread 사용
async fn main() {
    println!("Starting tasks...");

    let tasks: Vec<_> = (0..10).map(|i| {
        task::spawn(async move {
            println!("Task {} started on {:?}", i, thread::current().id());

            tokio::time::sleep(Duration::from_millis(500)).await;
            
            println!("Task {} completed on {:?}", i, thread::current().id());
        })
    }).collect();

    for t in tasks {
        let _ = t.await;
    }

    println!("All tasks completed.")
}
