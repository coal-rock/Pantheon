#[derive(Clone, Default)]
pub struct Statistics {
    pub packets_sent: u64,
    pub packets_recv: u64,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub latencies: Vec<u128>,
}

impl Statistics {
    pub fn log_send(&mut self, len: usize) {
        self.packets_sent += 1;
        self.bytes_sent += len as u64;
    }

    pub fn log_recv(&mut self, len: usize) {
        self.packets_recv += 1;
        self.bytes_recv += len as u64;
    }

    pub fn log_latency(&mut self, latency: u128) {
        self.latencies.push(latency);
    }

    pub fn get_total_traffic(&self) -> u64 {
        self.bytes_recv + self.bytes_sent
    }

    pub fn get_average_latency(&self) -> f32 {
        // prevent divide by zero
        if self.latencies.len() == 0 {
            return 0.0;
        }

        self.latencies.iter().sum::<u128>() as f32 / self.latencies.len() as f32
    }
}
