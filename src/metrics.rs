// 基本功能：inc/dec/snapshot

use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)] // 在使用 Arc 后，Clone的意义与使用前不一样，是 Arc::Clone，在多线程下对 data 的引用计数
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 内部可变，不再需要 mut
    // pub fn inc(&mut self, key: impl Into<String>) {
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // let counter = self.data.entry(key.into()).or_insert(0);
        // *counter += 1;

        // 首先要先拿到一个 lock
        // 返回的 MutexGuard 是一个 owned data，所以可以声明成 mut
        // MutexGuard 里面的东西可以直接操作，因为实现了 Deref 和 DerefMut
        // 整个过程对于self来说，它访问的都是只读的数据
        let mut data: std::sync::MutexGuard<HashMap<String, i64>> =
            self.data.lock().map_err(|e| anyhow!(e.to_string()))?;

        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
