use core::time::Duration;
use fastping_rs::PingResult::{Idle, Receive};
use influent::measurement::{Measurement, Value};

#[derive(Debug)]
pub struct PingResult {
    rtt: Duration,
    loss: f32
}
impl PingResult {
    pub fn new() -> PingResult {
        PingResult { rtt: Duration::from_millis(0), loss: 0.0 }
    }

    pub fn handle(&mut self, res: fastping_rs::PingResult) {
        match res {
            Idle{addr: _} => self.loss += 1.0,
            Receive{addr: _, rtt} => self.rtt += rtt
        }
    }

    pub fn rtt(&self) -> Duration { self.rtt }
    pub fn loss(&self) -> f32 { self.loss }

    pub fn update(&mut self, nping: u32) -> &mut PingResult {
        let fping = nping as f32;
        let loss = self.loss as u32;
        let div = if loss >= nping { 1 } else { nping - loss };

        self.rtt /= div;
        self.loss *= 100.0 / fping;

        self
    }

    pub fn into_measurements(&self, host: String) -> Vec<Measurement> {
        let mut rtt = Measurement::new("ttl");
        rtt.add_field("value",
                      Value::Float(self.rtt.as_micros() as f64 / 1000.0));
        rtt.add_tag("host", host.clone());
        rtt.add_tag("srchost", "mac");

        let mut loss = Measurement::new("loss");
        loss.add_field("value", Value::Float(self.loss as f64));
        loss.add_tag("host", host);
        loss.add_tag("srchost", "mac");

        vec![rtt, loss]
    }
}
