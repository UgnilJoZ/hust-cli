use reqwest::blocking::get;
use crate::error::Result;

#[derive(Deserialize, Debug)]
pub struct BridgeDevice {
	#[serde (rename = "UDN")]
	pub udn: String,
	#[serde (rename = "deviceType")]
	pub device_type: String,
	pub manufacturer: String,
	#[serde (rename = "modelName")]
	pub model_name: String,
	#[serde (rename = "modelDescription")]
	pub model_description: String,
	#[serde (rename = "serialNumber")]
	pub serial_number: String,
	#[serde (rename = "friendlyName")]
	pub friendly_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Bridge {
	#[serde (rename = "URLBase")]
	pub url_base: String,
	pub device: BridgeDevice,
}

impl Bridge {
	pub fn from_description_url(url: String) -> Result<Bridge> {
		let response = get(&url)?.text()?;
		let bridge: Bridge = serde_xml::from_str(&response)?;
		Ok(bridge)
	}
}