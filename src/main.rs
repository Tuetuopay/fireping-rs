extern crate env_logger;
// #[macro_use]
// extern crate log;

use fastping_rs::Pinger;
use timer::Timer;
use chrono::Duration;
use nix::unistd::{setuid, Uid};
use std::sync::mpsc;
use std::env;
use influent::client::Credentials;

mod log;
mod ping_result;
use ping_result::PingResult;
mod pinger;
use pinger::run_ping;
mod db;
use db::{Db, sqlitedb::SqliteDb, yamldb::YamlDb};
mod tsdb;
use tsdb::Tsdb;

static TIMEOUT: u64 = 100;
static TPING: i64 = 1000;
static NPING: u32 = 5;

#[derive(Debug, Clone)]
pub struct Target {
    name: String,
    target: String
}

#[derive(Debug)]
pub enum Error {
    YamlDbError(db::yamldb::Error),
    SqliteDbError(rusqlite::Error),
}

impl From<db::yamldb::Error> for Error {
    fn from(e: db::yamldb::Error) -> Self { Self::YamlDbError(e) }
}
impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self { Self::SqliteDbError(e) }
}

fn getenv(name: &str, default: &str) -> String {
    match env::var(&name) { Ok(s) => s, Err(_e) => default.to_string() }
}

fn main() -> Result<(), Error> {
    if !Uid::current().is_root() {
        match setuid(Uid::from_raw(0)) {
            Err(e) => panic!("Error switching to root: {}", e),
            _ => ()
        }
    }

    let influx_host = getenv("INFLUXDB_HOST", "localhost");
    let influx_cred = Credentials {
        username: &getenv("INFLUXDB_USER", "influser"),
        password: &getenv("INFLUXDB_PASS", "inflpass"),
        database: &getenv("INFLUXDB_DB", "influxdb")
    };
    let tsdb = Tsdb::new(&influx_host, influx_cred);

    env_logger::init();
    let (pinger, results) = match Pinger::new(Some(TIMEOUT), None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e)
    };

    let ips = if let Ok(db) = YamlDb::new() {
        db.targets()?
    }
    else if let Ok(db) = SqliteDb::new() {
        db.init()?;
        db.targets()?
    }
    else {
        panic!("No valid db found")
    };

    println!("Loaded from db: {:?}", ips);

    for ip in &ips {
        pinger.add_ipaddr(&ip.target);
    }

    let (results_tx, results_rx) = mpsc::channel();

    // Create timer
    let timer = Timer::new();
    let ips_clone = ips.to_vec();
    let _guard = timer.schedule_repeating(
        Duration::milliseconds(TPING),
        move || run_ping(&pinger, &results, &ips_clone, &results_tx, NPING));

    loop {
        match results_rx.recv() {
            Ok((res, timestamp)) => {
                println!("Summary:");
                log::print_summary(&ips, &res);
                // tsdb.push_results(&ips, res, timestamp as i64);
            },
            Err(_) => panic!("Timer thread disconnected!")
        }
    }
}
