struct LiteGateway {
    state: i64,
}

impl LiteGateway {
    fn new(seed: i64) -> Self {
        LiteGateway { state: seed }
    }

    fn load_engine(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 48) % 997;
        }
        total
    }
}

fn main() {
    let obj = LiteGateway::new(48);
    println!("{}", obj.load_engine(48));
}
