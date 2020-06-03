use std::io::{Error, ErrorKind};
use url::{Url, ParseError};

pub fn source(url: &str) -> Result<(), ParseError> {
	let supported_urls = ["api.spotify.com"];
	let parsed = Url::parse(url)?;

	let base_url = match parsed.host_str() {
		Some(s) => s,
		None => ""
	};

	if supported_urls.contains(&base_url) {
		return Ok(());
	}

	Err(ParseError::IdnaError)
}

pub fn destination(destination: &str) -> Result<(), Error> {
	let supported_destinations = ["deezer", "spotify"];
    if supported_destinations.contains(&destination) {
    	return Ok(());
    }

    Err(Error::new(ErrorKind::NotFound, "Destination option not yet supported"))
}
