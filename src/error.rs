#[derive(Deserialize, Debug)]
pub struct ApiError {
    #[serde(rename = "type")]
    error_type: u16,
    address: String,
    description: String,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Xml(serde_xml::Error),
    ApiErrors(Vec<ApiError>),
	Json(serde_json::Error),
    NoBridgeFound,
    Arbitrary(String),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    }
}

impl From<serde_xml::Error> for Error {
    fn from(error: serde_xml::Error) -> Error {
        Error::Xml(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::Json(error)
    }
}

impl From<Vec<ApiError>> for Error {
    fn from(errors: Vec<ApiError>) -> Error {
        Error::ApiErrors(errors)
    }
}

pub type Result<V> = std::result::Result<V, Error>;
