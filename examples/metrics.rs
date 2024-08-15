use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    println!("{:?}", metrics.snapshot());

    // 如果只是简单粗暴地用 clone，那么就跟主线程分离了，没有共享到同一个 metrics
    // 解决方法：Mutex
    // 但是 Mutex 是无法被 clone 的，想要在跨线程时使用，要加上 Arc
    // Mutex 本身是只读的，但是使用 lock 后，是一个 MutexGuard（此时不能Send），相当于拿到锁后，可以去数据进行修改，即所谓的"内部可变性"
    // 当 lock 的scope退出后，lock 会被丢弃，Mutex 就会变成 unlock 状态。如果一个线程被 lock 住了，另一个想要访问，只能等
    for idx in 0..N {
        task_worker(idx, metrics.clone())?; // 相当于 Metrics {data: Arc::Clone(&metrics)}
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics.snapshot());
    }
}

// 两种思路
// 1. 需要把 metrics 传到 worker 里面去
// 2. metrics table 是一个全局的 table
fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));

            metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
        }
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);

            // 这里要单独处理这个 ? ，因为 metrics.inc 返回 Result，而 thread::spawn 是返回 ()，返回值不一样
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
