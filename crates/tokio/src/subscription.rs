use std::{future::Future, pin::Pin};

use std::task::{Context, Poll};

#[derive(Debug, Clone)]
pub struct Subscription {
    // room: Arc<Room>,
    inner: Option<String>,
    next_event: u16,
}

impl Subscription {}

impl Future for Subscription {
    type Output = (u16, String);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(msg) = &self.inner {
            return Poll::Ready((self.next_event.clone(), msg.to_string()))
        }
        self.next_event += 1;
        self.inner = Some("changed".to_string());
        // Wake this task to be polled again.
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let sub = Subscription { inner: None, next_event: 100 };
    println!("1. {:?}", sub.clone());
    let (inner, msg) = sub.await;
    println!("2. {:?} {:?}", inner, msg);
}
