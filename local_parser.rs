struct FastEngine {
    state: i64,
}

impl FastEngine {
    fn new(seed: i64) -> Self {
        FastEngine { state: seed }
    }

    fn handle_dispatcher(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 38) % 997;
        }
        value
    }
}

fn main() {
    let obj = FastEngine::new(38);
    println!("{}", obj.handle_dispatcher(38));
}
