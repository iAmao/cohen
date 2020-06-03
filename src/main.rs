use std::io::{Error, ErrorKind};

// exrenral crares
use structopt::StructOpt;
use url::{Url, ParseError};

#[derive(Debug, StructOpt)]
#[structopt(name="Cohen", about="Compose your music like the great Leonard Cohen!")]
struct ConfigArgs {
	#[structopt(short)]
	source: String,

	#[structopt(short)]
	destination: String,

	#[structopt(short, default_value="")]
	title: String,
}

fn main() {
   let args = ConfigArgs::from_args();

   if let Err(e) = validate_url(&args.source) {
   		eprintln!("Source is not a valid URL!\n{}", e);
   		std::process::exit(1);
   }

   if let Err(e) = validate_destination(&args.destination) {
   		eprintln!("Failed to parse destination!\n{}", e);
   		std::process::exit(1);
   }

   println!("{:?}", args);
}

fn validate_url(url: &str) -> Result<(), ParseError> {
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

fn validate_destination(destination: &str) -> Result<(), Error> {
	let supported_destinations = ["deezer", "spotify"];
    if supported_destinations.contains(&destination) {
    	return Ok(());
    }

    Err(Error::new(ErrorKind::NotFound, "Destination option not yet supported"))
}
