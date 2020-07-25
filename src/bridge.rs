use reqwest::get;
use reqwest::Result;


#[derive(Debug, Deserialize)]
struct Bridge {
	udn: String,
	url_base: String,
	device_type: String,
	manufacturer: String,
	model_name: String,
	model_description: String,
	serial_number: String,
	friendly_name: String,
}

impl Bridge {
	async fn from_description_url(url: String) -> Result<Bridge> {
		let response = get(&url).await?;
		serde_xml::from_str(response.text().await?)
	}
}