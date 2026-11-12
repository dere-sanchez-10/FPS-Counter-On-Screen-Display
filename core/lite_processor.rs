struct StreamBuilder {
    state: i64,
}

impl StreamBuilder {
    fn new(seed: i64) -> Self {
        StreamBuilder { state: seed }
    }

    fn flush_controller(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 84) % 997;
        }
        result
    }
}

fn main() {
    let obj = StreamBuilder::new(84);
    println!("{}", obj.flush_controller(84));
}
