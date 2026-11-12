struct StreamEngine {
    state: i64,
}

impl StreamEngine {
    fn new(seed: i64) -> Self {
        StreamEngine { state: seed }
    }

    fn render_processor(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 57) % 997;
        }
        value
    }
}

fn main() {
    let obj = StreamEngine::new(57);
    println!("{}", obj.render_processor(57));
}
