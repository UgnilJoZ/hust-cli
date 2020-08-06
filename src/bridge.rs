use std::collections::HashMap;
use reqwest::blocking::{get};
use crate::error::{Result,ApiError};

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

#[derive(Deserialize, Debug)]
pub enum ApiResponseSection {
	#[serde (rename = "error")]
	Err(ApiError),
	#[serde (rename = "success")]
	Success(HashMap<String, String>)
}

impl Bridge {
	pub fn from_description_url(url: String) -> Result<Bridge> {
		let response = get(&url)?.text()?;
		let bridge: Bridge = serde_xml::from_str(&response)?;
		Ok(bridge)
	}

	pub fn register_user(&self) -> Result<String> {
		let client = reqwest::blocking::Client::new();
		let mut url = self.url_base.clone();
		let mut params = HashMap::new();
		params.insert("devicetype", "Hust Hue API client");
		url.push_str("api");
		let response = client
			.post(&url)
			.json(&params)
			.send()?;
		let response: Vec<ApiResponseSection> = serde_json::from_reader(response)?;
		let mut errors = vec!();
		let mut success = None;
		for section in response {
			match section {
				ApiResponseSection::Err(e) => errors.push(e),
				ApiResponseSection::Success(hashmap) => success = Some(hashmap),
			}
		}
		if let Some(hashmap) = success {
			if let Some(username) = hashmap.get("username") {
				return Ok(username.to_string())
			}
		}
		Err(errors)?
	}
}