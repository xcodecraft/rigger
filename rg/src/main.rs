#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate rg_core;

#[macro_use]
extern crate clap;

use clap::{Arg, App};

fn main() {
	pretty_env_logger::init();
	rg_core::rg_main();
//	parse();
}


fn parse() {
	//rg conf start -e dev -s init
	//rg conf |start|stop|clean
	//rg rc
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about("rg is rigger simple name")
		.arg(Arg::with_name("env")
			.short("e")
			.long("env")
			.takes_value(true)
			.requires("system")
			.help("env"))
		.arg(Arg::with_name("system")
			.short("s")
			.long("system")
			.takes_value(true)
			.help("system"))
		.arg(Arg::with_name("conf")
			.help("rigger conf in project")
			.multiple(true)
			.possible_values(&["conf", "clean", "start", "stop", "rc"]))
		.get_matches();

	if let Some(config) = matches.values_of("conf") {
		let confs = Some(config).unwrap().collect::<Vec<_>>();
		println!("confs is {:?}", confs);
	}

	if let Some(config) = matches.value_of("action") {
		println!("A action is: {}", config);
	}

	if let Some(config) = matches.value_of("env") {
		println!("A env is: {}", config);
	}

	if let Some(config) = matches.value_of("system") {
		println!("A system is: {}", config);
	}
}
