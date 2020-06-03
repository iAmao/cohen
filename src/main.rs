use structopt::StructOpt;


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
   println!("{:?}", args);
}
