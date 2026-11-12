struct HybridProvider {
    state: i64,
}

impl HybridProvider {
    fn new(seed: i64) -> Self {
        HybridProvider { state: seed }
    }

    fn load_dispatcher(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 53) % 997;
        }
        result
    }
}

fn main() {
    let obj = HybridProvider::new(53);
    println!("{}", obj.load_dispatcher(53));
}
