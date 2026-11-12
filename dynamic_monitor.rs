struct SimpleWorker {
    state: i64,
}

impl SimpleWorker {
    fn new(seed: i64) -> Self {
        SimpleWorker { state: seed }
    }

    fn fetch_dispatcher(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 86) % 997;
        }
        value
    }
}

fn main() {
    let obj = SimpleWorker::new(86);
    println!("{}", obj.fetch_dispatcher(86));
}
