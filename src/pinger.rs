use fastping_rs::Pinger;
use fastping_rs::PingResult::{Idle, Receive};
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::time::SystemTime;

use crate::{PingResult, Target};

pub fn run_ping(pinger: &Pinger,
                results: &Receiver<fastping_rs::PingResult>,
                ips: &Vec<Target>,
                results_tx: &Sender<(HashMap<String, PingResult>, u128)>,
                nping: u32) {
    let mut res: HashMap<String, PingResult> = HashMap::new();

    let ping_count = nping as usize * ips.len();

    // Fill maps
    for ip in ips {
        res.insert(ip.target.to_string(), PingResult::new());
    }

    pinger.run_pinger();
    for _ in 0..ping_count {
        match results.recv() {
            Ok(result) => {
                let addr = match result {
                    Idle{addr} => addr,
                    Receive{addr, rtt: _} => addr
                };
                res.entry(addr.to_string()).and_modify(|e| e.handle(result));
            },
            Err(_) => panic!("Worker threads disconnected!"),
        }
    }
    pinger.stop_pinger();

    // Compute values
    for (_, r) in res.iter_mut() {
        r.update(nping);
    }

    let time_ns = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos();

    // Forward it to main thread
    results_tx.send((res, time_ns)).unwrap();
}
