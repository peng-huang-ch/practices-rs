use tokio::time::{sleep, timeout, Duration};

#[allow(dead_code)]
async fn tokio_timeout() {
    let result = timeout(Duration::from_secs(5), sleep(Duration::from_secs(10))).await;

    match result {
        Ok(_) => println!("Task finished in time"),
        Err(_) => println!("Task timed out"),
    }
}

#[tokio::test]
async fn test_tokio_timeout() {
    tokio_timeout().await;
}
