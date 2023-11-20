pub mod executor;
pub mod timer_future;

#[cfg(test)]
mod test_async_future {
    use std::time::Duration;

    use crate::{executor, timer_future::TimerFuture};

    #[test]
    fn test_timer_future() {
        let (executor, spawner) = executor::new_executor_and_spawner();
        // 生成一个任务
        spawner.spawn(async {
            println!("ready go!");
            let timer_future = TimerFuture::new(Duration::from_secs(2));
            timer_future.await;
            println!("done!");
        });
        drop(spawner);

        executor.run();
        drop(executor);
    }
}
