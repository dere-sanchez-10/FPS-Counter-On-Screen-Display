struct SmartCache {
    state: i64,
}

impl SmartCache {
    fn new(seed: i64) -> Self {
        SmartCache { state: seed }
    }

    fn sync_cache(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 37) % 997;
        }
        value
    }
}

fn main() {
    let obj = SmartCache::new(37);
    println!("{}", obj.sync_cache(37));
}
