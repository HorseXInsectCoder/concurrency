// 基本功能：inc/dec/snapshot

use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)] // 在使用 Arc 后，Clone的意义与使用前不一样，是 Arc::Clone，在多线程下对 data 的引用计数
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // 内部可变，不再需要 mut
    // pub fn inc(&mut self, key: impl Into<String>) {
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // 写的地方用 write
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;

        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    // 是否有必要存在 snapshot 应该这样考虑：如果要做很多读操作的话，做 snapshot 是有必要的。
    // 如果只是展示，那么可以使用另外一种方法。并不用 clone，拿到读锁后，直接把数据读出来
    // 这就取决于 clone 的效率更高还是读取出来展示的效率高
    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read() // 读的地方用 read
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 由于 fmt::Error 没有办法携带更多信息，只能把当前 error 丢弃
        let data = self.data.read().map_err(|_e| fmt::Error {})?;
        for (key, value) in data.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
