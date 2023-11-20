use tokio::time;

#[allow(dead_code)]
async fn some_operation() -> String {
    let mut interval = time::interval(time::Duration::from_secs(2));
    for _i in 0..5 {
        interval.tick().await;
        println!("now is some_operation");
    }
    "some_operation".to_string()
}

#[cfg(test)]
mod async_samples {
    use tokio::time::sleep;

    use super::some_operation;
    use std::time::Duration;

    #[tokio::main]
    #[test]
    async fn test_some_operation() {
        use tokio::sync::oneshot;

        let (mut tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();

        tokio::spawn(async {
            // 等待 `some_operation` 的完成
            // 或者处理 `oneshot` 的关闭通知
            tokio::select! {
                val = some_operation() => {
                    let _ = tx1.send(val);
                }
                _ = tx1.closed() => {
                    // 收到了发送端发来的关闭信号
                    // `select` 即将结束，此时，正在进行的 `some_operation()` 任务会被取消，任务自动完成，
                    // tx1 被释放
                }
            }
        });

        tokio::spawn(async {
            sleep(Duration::from_millis(4 * 1000)).await;
            let _ = tx2.send("two");
        });

        tokio::select! {
            val = rx1 => {
                println!("rx1 completed first with {:?}", val);
            }
            val = rx2 => {
                println!("rx2 completed first with {:?}", val);
            }
        }
    }
}
