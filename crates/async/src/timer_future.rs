use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    pub shared_state: Arc<Mutex<SharedState>>,
}

pub struct SharedState {
    /// 定时(睡眠)是否结束
    completed: bool,
    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 通过检查共享状态，来确定定时器是否已经完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            // 如果定时器已经完成，则直接返回`Ready`
            return Poll::Ready(());
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        // new tread pool
        let thread_pool = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_pool.lock().unwrap();
            shared_state.completed = true;
            shared_state.waker.take().map(|waker| waker.wake());
            // if let Some(waker) = shared_state.waker.take() {
            //     waker.wake();
            // }
        });
        TimerFuture { shared_state }
    }
}
