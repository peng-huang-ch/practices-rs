use futures::future;
use std::{io::Result, time::Instant};
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
pub async fn thread_1() -> Result<()> {
    let now = Instant::now();
    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(tokio::spawn(bg_task(i)));
        // handles.push(bg_task(i));
    }

    std::thread::sleep(Duration::from_millis(100));
    println!("Finished time-consuming task.");

    for handle in handles {
        handle.await?;
    }
    println!("总耗时：{} ms", now.elapsed().as_millis());
    Ok(())
}

#[allow(dead_code)]
pub async fn thread_2() -> Result<()> {
    let now = Instant::now();
    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(tokio::spawn(bg_task(i)));
    }

    std::thread::sleep(Duration::from_millis(100));
    println!("Finished time-consuming task.");

    future::join_all(handles).await;

    println!("总耗时：{} ms", now.elapsed().as_millis());
    Ok(())
}

#[allow(dead_code)]
pub async fn bg_task(i: u64) {
    let millis = i.clone();
    println!("Task {} sleeping for {} s.", i, millis);
    sleep(Duration::from_secs(millis)).await;
    println!("Task {} stopping.", i);
}

#[tokio::main]
#[test]
async fn test_thread_1() -> Result<()> {
    thread_1().await
}

#[tokio::main]
#[test]
async fn test_thread_2() -> Result<()> {
    thread_1().await
}
