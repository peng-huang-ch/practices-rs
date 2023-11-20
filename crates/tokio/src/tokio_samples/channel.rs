use tokio::sync::mpsc;

#[allow(dead_code)]
async fn tokio_channel() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            if tx.send(i).await.is_err() {
                break;
            }
        }
        // println!("val is {}", val);
    });

    while let Some(v) = rx.recv().await {
        println!("{}", v);
    }
}

#[tokio::main]
#[test]
async fn test_tokio_channel() {
    tokio_channel().await;
}
