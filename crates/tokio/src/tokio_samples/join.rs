use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};
use tokio::{time, time::sleep};

#[allow(dead_code)]
async fn task_one() {
    sleep(time::Duration::from_millis(1)).await
}

#[allow(dead_code)]
async fn task_two() {
    sleep(time::Duration::from_millis(2)).await
}

#[allow(dead_code)]
async fn task_three() {
    sleep(time::Duration::from_millis(3)).await
}

#[allow(dead_code)]
async fn task_self() {
    // sleep(time::Duration::from_millis(1)).await
}

#[allow(dead_code)]
async fn race_tasks() {
    // fuse 将返回 FuseFuture，该 Future 包装一个子Future
    // 当子 Future 返回 Pending 时，返回Pending
    // 当子 Future 第一次返回 Ready 时，返回Ready
    // 此后 不会调用再调用 子Future，且永远返回Pending
    let t1 = task_one();
    let t2 = task_two().fuse();
    let t3 = task_three().fuse();
    // let t4 = task_self().fuse();
    pin_mut!(t1, t2, t3);

    select! {
        () = t1 => println!("task one completed first"),
        () = t2 => println!("task two completed first"),
        () = t3 => println!("task two completed three"),
        // () = t4 => println!("task self completed three"),
        // () = task_self().fuse() => println!("task self completed second"),
        () = task_two().fuse() => println!("task third completed second")
    }
}

#[cfg(test)]
mod race_sample {

    use super::race_tasks;

    #[tokio::main]
    #[test]
    async fn test_race_tasks() {
        race_tasks().await;
    }
}
