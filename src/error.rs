#[derive(Debug)]
pub enum Error {
	Io(std::io::Error),
	Reqwest(reqwest::Error),
	Xml(serde_xml::Error),
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error)	-> Error {
		Error::Io(error)
	}
}

impl From<reqwest::Error> for Error {
	fn from(error: reqwest::Error)	-> Error {
		Error::Reqwest(error)
	}
}

impl From<serde_xml::Error> for Error {
	fn from(error: serde_xml::Error)	-> Error {
		Error::Xml(error)
	}
}

pub type Result<V> = std::result::Result<V, Error>;