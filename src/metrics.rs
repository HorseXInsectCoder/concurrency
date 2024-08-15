use anyhow::Result;
use dashmap::DashMap;
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)] // 在使用 Arc 后，Clone的意义与使用前不一样，是 Arc::Clone，在多线程下对 data 的引用计数
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // iter 出来是一个 entry RefMulti<'_, String, i64>
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
