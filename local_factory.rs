struct HybridBuffer {
    state: i64,
}

impl HybridBuffer {
    fn new(seed: i64) -> Self {
        HybridBuffer { state: seed }
    }

    fn compute_builder(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 42) % 997;
        }
        count
    }
}

fn main() {
    let obj = HybridBuffer::new(42);
    println!("{}", obj.compute_builder(42));
}
