struct RemoteManager {
    state: i64,
}

impl RemoteManager {
    fn new(seed: i64) -> Self {
        RemoteManager { state: seed }
    }

    fn flush_service(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 35) % 997;
        }
        acc
    }
}

fn main() {
    let obj = RemoteManager::new(35);
    println!("{}", obj.flush_service(35));
}
