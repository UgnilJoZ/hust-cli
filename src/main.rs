#![feature(duration_zero)]
use std::time::Duration;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_xml;
extern crate reqwest;
mod error;
use error::Result;
mod discovery;
use discovery::find_bridges;
mod bridge;

fn main() -> Result<()> {
    for bridge in find_bridges(Duration::from_secs(2))? {
        println!("{:?}", bridge?);
    }
    Ok(())
}
