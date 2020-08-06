#![feature(duration_zero)]
use std::time::Duration;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_xml;
use structopt::StructOpt;
mod error;
use error::Result;
mod discovery;
use discovery::find_bridges;
mod bridge;
pub use bridge::Bridge;
mod config;
use config::Config;

#[derive(StructOpt)]
#[structopt(name = "hust", about = "Hue bridge client in Rust")]
enum Opt {
    Discover {
        /// Timeout in seconds. Float values are accepted.
        #[structopt(short, long, default_value = "2")]
        timeout: f64,

        /// Maximum devices to find
        #[structopt(short, long)]
        max: Option<usize>,
    }
}

fn main() -> Result<()> {
    let mut config = Config::load().unwrap_or_default();
    let opt = Opt::from_args();
    match opt {
        Opt::Discover{timeout, max} => {
            let bridge_iter = find_bridges(Duration::from_secs_f64(timeout))?
                .inspect(|b| println!("{:?}", b))
                .filter_map(|b| b.ok());
            config.bridges = if let Some(max_devices) = max {
                bridge_iter.take(max_devices).collect()
            } else {
                bridge_iter.collect()
            };
        }
    }
    Ok(())
}
