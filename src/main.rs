#![feature(duration_zero)]
use std::time::Duration;
use std::io::Result;

extern crate serde;
extern crate serde_xml;
#[macro_use] extern crate serde_derive;
extern crate reqwest;

mod discovery;
use discovery::discover;
mod bridge;

fn main() -> Result<()> {
    for url in discover(1, Duration::from_secs(2))? {
        println!("{}", url);
    }
    Ok(())
}
