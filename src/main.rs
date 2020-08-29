#![feature(duration_zero)]
use std::time::Duration;
extern crate structopt;
use structopt::StructOpt;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
mod config;
extern crate hust;
use hust::{find_bridges, Result, Error};
use config::Config;

/// The CLI options.
/// 
/// These are the subcommands and flags.
#[derive(StructOpt)]
#[structopt(name = "hust", about = "Hue bridge client in Rust")]
enum Opt {
    /// Discover hue bridges in the network
    Discover {
        /// Timeout in seconds. Float values are accepted.
        #[structopt(short, long, default_value = "2")]
        timeout: f64,

        /// Maximum devices to find
        #[structopt(short, long)]
        max: Option<usize>,
    },
    /// Register a user on a bridge
    Register {
        bridge: Option<String>,
    },
    /// Several commands for light control
    Light(LightOpt),
}

#[derive(StructOpt)]
struct LightOpt {
    #[structopt(short, long)]
    bridge: Option<String>,
    #[structopt(subcommand)]
    subcommand: LightCommand,
}

#[derive(StructOpt)]
enum LightCommand {
    List,
    Switch {
        #[structopt(short, long, required(true))]
        light: String,
        adjective: String,
    }
}

/// Executes a CLI app to use this library.
fn main() -> Result<()> {
    let mut config = Config::load().unwrap_or_default();
    let opt = Opt::from_args();
    match opt {
        Opt::Discover{timeout, max} => {
            let bridge_iter = find_bridges(Duration::from_secs_f64(timeout))?
                .inspect(|b| match b {
                    // serde error won't happen, therefore unwrap
                    Ok(b) => println!("{}", serde_json::to_string_pretty(&b).unwrap()),
                    Err(e) => println!("{:?}", e),
                })
                .filter_map(|b| b.ok());
            config.bridges = if let Some(max_devices) = max {
                bridge_iter.take(max_devices).collect()
            } else {
                bridge_iter.collect()
            };
            config.save()?;
            println!("Config file written: {:?}", config::get_config_file());
        }

        Opt::Register{bridge: bridge_name} => {
            let bridge = if let Some(bridge_id) = bridge_name {
                config.bridges
                .iter()
                .filter(|&b| b.url_base == bridge_id)
                .next()
            } else {
                config.bridges.get(0)
            }.ok_or(Error::NoBridgeFound)?;

            let username = bridge.register_user()?;

            println!("User {} registered.", bridge.device.udn);
            config.usernames.insert(bridge.device.udn.clone(), username);
            config.save()?;
        }

        Opt::Light(light_options) => {
            let bridge = if let Some(specified_bridge) = light_options.bridge {
                config.bridges
                .iter()
                .filter(|b| b.device.friendly_name == specified_bridge || b.device.udn == specified_bridge)
                .next()
            } else {
                config.bridges
                .get(0)
            }
            .ok_or(Error::NoBridgeFound)?;

            match light_options.subcommand {
                LightCommand::List => {
                    let username = config.usernames.get(&bridge.device.udn).unwrap();
                    for (number, light) in bridge.get_all_lights(&username)? {
                        println!("{}:\t{}", number, light.name);
                        println!("\t{}", light.uniqueid);
                        let switched = if light.state.on {
                            "on"
                        } else {
                            "off"
                        };
                        println!("\t{}, bri: {}, col: {}", switched, light.state.brightness, light.state.ct);
                        println!("\t{}", light.productid);
                        println!();
                    }
                }
                LightCommand::Switch{light, adjective} => {
                    let username = config.usernames.get(&bridge.device.udn).unwrap();
                    if adjective == "on" {
                        bridge.switch_light(username, &light, true)?
                    } else if adjective == "off" {
                        bridge.switch_light(username, &light, false)?
                    } else {
                        return Err(Error::Arbitrary("Verb has to be 'on' or 'off'.".to_string()))
                    };
                }
            }
        }
    }
    Ok(())
}
