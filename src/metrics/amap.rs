use anyhow::Result;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

// 在实现 new 的时候要注意，这时不能创建一个空的HashMap，因为这个HashMap是不可写的，在 new 的时候就应该要知道要监控什么内容(name)
impl AmapMetrics {
    // metrics_name参数是 iterator
    pub fn new(metrics_name: &[&'static str]) -> Self {
        let map = metrics_name
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect::<HashMap<_, _>>();
        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("key {} not found", key))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
