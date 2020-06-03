use structopt::StructOpt;

pub mod validator;

#[derive(Debug, StructOpt)]
#[structopt(name="Cohen", about="Compose your music like the great Leonard Cohen!")]
pub struct ConfigArgs {
	#[structopt(short)]
	pub source: String,

	#[structopt(short)]
	pub destination: String,

	#[structopt(short, default_value="")]
	pub title: String,
}
