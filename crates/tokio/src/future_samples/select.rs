use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Future;
use tokio::sync::oneshot;

#[allow(dead_code)]
async fn some_operation() -> String {
    // Compute value here
    "some_operation".to_string()
}

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::net::TcpStream;
    use tokio::sync::oneshot;

    #[tokio::main]
    #[test]
    async fn test_select() {
        let (tx1, rx1) = oneshot::channel();
        let (_tx2, rx2) = oneshot::channel();
        tx1.send("hello").unwrap();
        MySelect { rx1, rx2 }.await;
    }

    #[tokio::main]
    #[test]
    async fn test_x() {
        let (mut tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();

        tokio::spawn(async {
            // Select on the operation and the oneshot's
            // `closed()` notification.
            tokio::select! {
                val = some_operation() => {
                    println!("some_operation completed with {:?}", val);
                    let _ = tx1.send(val);
                }
                _ = tx1.closed() => {
                    println!("tx1 closed");
                    // `some_operation()` is canceled, the
                    // task completes and `tx1` is dropped.
                }
            }
        });

        tokio::spawn(async {
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

    #[tokio::main]
    #[test]
    async fn test_tokio_select() {
        let (tx, rx) = oneshot::channel();

        // Spawn a task that sends a message over the oneshot
        tokio::spawn(async move {
            tx.send("done").unwrap();
        });

        tokio::select! {
            socket = TcpStream::connect("localhost:3465") => {
                println!("Socket connected {:?}", socket);
            }
            msg = rx => {
                println!("received message first {:?}", msg);
            }
        }
    }

    #[tokio::main]
    #[test]
    async fn test_unfold() -> Result<(), anyhow::Error> {
        use futures::sink::{self, SinkExt};
        let unfold = sink::unfold(1, |mut sum, i: i32| async move {
            println!("i = {}, sum = {}", i, sum);

            sum += i;
            println!("i = {}, sum = {}", i, sum);
            Ok::<_, anyhow::Error>(sum)
        });
        tokio::pin!(unfold);
        let result = unfold.send(5).await?;
        println!("{:?}", result);
        Ok(())
    }
}
