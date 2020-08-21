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

/// An object to communicate with a bridge.
/// 
/// The struct attributes contain the static properties of the bridge.
/// 
/// The methods can be used for communication like commands.
#[derive(Deserialize, Serialize, Debug)]
pub struct Bridge {
    #[serde(rename = "URLBase")]
    /// The base URL of the bridge, which all `/api/` resources are below of.
    pub url_base: String,
    /// The device properties of this bridge.
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
    /// Creates a Bridge object from a description URL like returned in SSDP discovery.
    pub fn from_description_url(url: String) -> Result<Bridge> {
        let response = get(&url)?.text()?;
        let bridge: Bridge = serde_xml::from_str(&response)?;
        Ok(bridge)
    }

    /// The unique but user-friendly name of the bridge.
    pub fn user_readable_identifier(&self) -> &str {
        &self.device.friendly_name
    }

    /// Register a user and return its name.
    /// 
    /// Save it to communicate further with the bridge, e.g. to switch lights.
    /// 
    /// Note that the button of the bridge has to be pressed.
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
    
    /// List all lights connected to this bridge
    /// 
    /// The listed lights are bundled with their state. You have to
    /// specify a user in order to be authenticated.
    pub fn get_all_lights(&self, user: &str) -> Result<HashMap<String, Light>> {
        let url = format!("{}api/{}/lights", self.url_base, user);
        let response = get(&url)?;
        Ok(serde_json::from_reader(response)?)
    }

    /// Switch light on / off.
    /// 
    /// `user` is the user you have to register with `register_user`.
    /// 
    /// `light` is the identifier of the light. All identifiers can
    /// be obtained by listing the dictionary keys of `get_all_lights`.
    /// 
    /// To switch the light off, please specify `on` as `false`.
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
