use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建 producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx); // 由于一开始就有一个tx，所以最后一共5个tx，这里要手动 drop 一个，否则 rx 无法结束

    // 创建 consumer
    // consumer 可以在主线程也可以是在子线程
    // producer 一直生产，不需要 join，但 consumer 需要
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        9654 // 子线程返回一个值
    });

    // 在主线程 join
    // anyhow 没有实现对 join handle 的处理，所以需要手动处理，使用 anyhow 构建一个 Error
    // 当通过 ? 没办法把一种错误转换为另一种错误的时候，需要使用 map_err 进行显式转换
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    println!("secret: {}", secret); // 主线程可以读取到子线程返回的值

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        // send 出错才会返回
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        // 当所有 producer drop 后，mpsc channel 就会被 drop 掉，然后 receiver 就会知道（然后结束）
        // 当加入退出条件后，就要考虑返回值了，可以直接 return，或者在最外面加一个 Ok
        if rand::random::<u8>() % 10 == 0 {
            println!("producer {} exit", idx);

            // break;
            return Ok(());
        }
    }
    // Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
