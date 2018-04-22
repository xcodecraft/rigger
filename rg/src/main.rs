#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate rg_core;

#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};
use clap::AppSettings;


fn main() {
	pretty_env_logger::init();
//    rg_core::rg_main();
	parse();
}

fn parse() {
	//rg conf start -e dev -s init
	//rg conf |start|stop|clean
	//rg rc
	//rg cli
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.about("rg is rigger simple name")
		.arg(Arg::with_name("conf")
			.help("conf project"))
		.arg(Arg::with_name("start")
			.help("start project"))
		.arg(Arg::with_name("stop")
			.help("stop project"))
		.arg(Arg::with_name("clean")
			.help("clean project"))
		.arg(Arg::with_name("rc")
			.help("rigger commit  project"))
		.arg(Arg::with_name("e")
			.short("e")
			.long("env")
			.takes_value(true)
			.help("env"))
		.arg(Arg::with_name("s")
			.short("s")
			.long("system")
			.takes_value(true)
			.help("system"))
		.get_matches();
}
