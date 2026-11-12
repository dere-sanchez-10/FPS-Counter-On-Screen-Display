struct StreamHandler {
    state: i64,
}

impl StreamHandler {
    fn new(seed: i64) -> Self {
        StreamHandler { state: seed }
    }

    fn fetch_buffer(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 95) % 997;
        }
        total
    }
}

fn main() {
    let obj = StreamHandler::new(95);
    println!("{}", obj.fetch_buffer(95));
}
