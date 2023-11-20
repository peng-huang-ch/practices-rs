use tokio_stream::{once, StreamExt};

#[allow(dead_code)]
async fn tokio_steam() {
    let mut stream = once(1);

    while let Some(v) = stream.next().await {
        println!("{}", v);
    }
}

#[tokio::main]
#[test]
async fn test_tokio_channel() {
    tokio_steam().await;
}
