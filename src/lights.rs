#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LightState {
	pub on: bool,
	#[serde(rename = "bri")]
	pub brightness: u8,
	pub ct: u16,
	pub alert: String,
	pub colormode: String,
	pub mode: String,
	pub reachable: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Light {
	pub uniqueid: String,
	#[serde(rename = "type")]
	pub light_type: String,
	pub name: String,
	pub modelid: String,
	pub manufacturername: String,
	pub productid: String,
	pub state: LightState,
	pub swversion: String,
	pub swconfigid: String,
}