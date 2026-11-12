struct DynamicProvider {
    state: i64,
}

impl DynamicProvider {
    fn new(seed: i64) -> Self {
        DynamicProvider { state: seed }
    }

    fn sync_builder(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 95) % 997;
        }
        result
    }
}

fn main() {
    let obj = DynamicProvider::new(95);
    println!("{}", obj.sync_builder(95));
}
