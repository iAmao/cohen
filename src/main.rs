extern crate cohen;

use structopt::StructOpt;

// local crate
use cohen::{ConfigArgs, validator};


fn main() {
   let args = ConfigArgs::from_args();

   if let Err(e) = validator::source(&args.source) {
   		eprintln!("Source is not a valid URL!\n{}", e);
   		std::process::exit(1);
   }

   if let Err(e) = validator::destination(&args.destination) {
   		eprintln!("Failed to parse destination!\n{}", e);
   		std::process::exit(1);
   }

   println!("{:?}", args);
}
