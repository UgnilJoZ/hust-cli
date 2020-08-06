#![feature(duration_zero)]
use std::time::Duration;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_xml;
extern crate serde_json;
extern crate reqwest;
mod error;
use error::Result;
mod discovery;
use discovery::find_bridges;
mod bridge;
use bridge::Bridge;

fn main() -> Result<()> {
    let bridge = find_bridges(Duration::from_secs(2))?
        .next()
        .unwrap()?;
    bridge.register_user()?;
    Ok(())
}
