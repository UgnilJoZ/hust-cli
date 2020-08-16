use crate::error::{ApiError, Result};
use crate::lights::Light;
use reqwest::blocking::get;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct BridgeDevice {
    #[serde(rename = "UDN")]
    pub udn: String,
    #[serde(rename = "deviceType")]
    pub device_type: String,
    pub manufacturer: String,
    #[serde(rename = "modelName")]
    pub model_name: String,
    #[serde(rename = "modelDescription")]
    pub model_description: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Bridge {
    #[serde(rename = "URLBase")]
    pub url_base: String,
    pub device: BridgeDevice,
}

#[derive(Deserialize, Debug)]
pub enum ApiResponseSection {
    #[serde(rename = "error")]
    Err(ApiError),
    #[serde(rename = "success")]
    Success(HashMap<String, serde_json::Value>),
}

impl Bridge {
    pub fn from_description_url(url: String) -> Result<Bridge> {
        let response = get(&url)?.text()?;
        let bridge: Bridge = serde_xml::from_str(&response)?;
        Ok(bridge)
    }

    pub fn user_readable_identifier(&self) -> &str {
        &self.device.friendly_name
    }

    pub fn register_user(&self) -> Result<String> {
        let client = reqwest::blocking::Client::new();
		let mut url = self.url_base.clone();
		url.push_str("api");
        let mut params = HashMap::new();
        params.insert("devicetype", "Hust Hue API client");
        let response = client.post(&url).json(&params).send()?;
        let response: Vec<ApiResponseSection> = serde_json::from_reader(response)?;
        let mut errors = vec![];
        let mut success = None;
        for section in response {
            match section {
                ApiResponseSection::Err(e) => errors.push(e),
                ApiResponseSection::Success(hashmap) => success = Some(hashmap),
            }
        }
        if let Some(hashmap) = success {
            if let Some(username) = hashmap.get("username") {
                return Ok(username.to_string());
            }
        }
		Err(errors)?
    }
    
    pub fn get_all_lights(&self, user: &str) -> Result<HashMap<String, Light>> {
        let url = format!("{}api/{}/lights", self.url_base, user);
        let response = get(&url)?;
        Ok(serde_json::from_reader(response)?)
    }

    pub fn switch_light(&self, user: &str, light: &str, on: bool) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}api/{}/lights/{}/state", self.url_base, user, light);
        let mut params = HashMap::new();
        params.insert("on", on);
        let response = client
            .put(&url)
            .json(&params)
            .send()?;
        let response: Vec<ApiResponseSection> = serde_json::from_reader(response)?;
        let mut errors = vec![];
        let success = response
            .into_iter()
            .any(|section|
                match section {
                    ApiResponseSection::Success(_) => true,
                    ApiResponseSection::Err(e) => {
                        errors.push(e);
                        false
                    }
                });
        if success {
            return Ok(())
        } 
        Err(errors)?
    }
}
