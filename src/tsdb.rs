use influent::{create_async_reqwest_client,
               client::{Client, Credentials, http::HttpClient},
               measurement::Measurement};
use std::collections::HashMap;
use futures::future::Future;

use crate::{Target, PingResult};

pub struct Tsdb<'a> {
    client: HttpClient<'a>
}

impl<'a> Tsdb<'a> {
    pub fn new(host: &'a str, creds: Credentials<'a>) -> Tsdb<'a> {
        Tsdb { client: create_async_reqwest_client(creds, vec![host]) }
    }

    pub fn push_results(&self, targets: &Vec<Target>,
                        results: HashMap<String, PingResult>,
                        timestamp: i64) {
        let mut measurements: Vec<Measurement> =
            targets.into_iter()
                   .flat_map(|t| results.get(&t.target)
                                        .unwrap()
                                        .into_measurements(t.name.clone())
                                        .into_iter())
                   .collect();
        for m in &mut measurements {
            m.set_timestamp(timestamp);
        }

        tokio::run(self.client.write_many(&measurements, None)
                              .map_err(|e| println!("write_many err: {:?}", e)))
    }
}
