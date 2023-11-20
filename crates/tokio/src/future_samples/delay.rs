use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::Instant,
};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() < self.when {
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;
            // Spawn a timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Delay;
    use std::time::{Duration, Instant};

    #[tokio::main]
    #[test]
    async fn test_delay() {
        let when: Instant = Instant::now() + Duration::from_millis(10);
        let future: Delay = Delay { when };
        let out = future.await;
        assert_eq!(out, "done");
    }
}
