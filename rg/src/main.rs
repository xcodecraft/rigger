#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate rg_core;
#[macro_use] extern crate clap;
use clap::{Arg, App};

use rg_core::* ;


fn main() {
    info!("rg runing") ;
	pretty_env_logger::init();
    let mut args = RGArgs::new();
	args_parse(&mut args);
    rg_core::rg_main(args);
}


//rg conf start -e dev -s init
//rg conf |start|stop|clean
//rg rc
fn args_parse(args : &mut RGArgs) {
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
		.arg(Arg::with_name("action")
			.help("rigger action ")
			.multiple(true)
			.possible_values(&["conf", "clean", "start", "stop", "conf,start"]))
		.get_matches();
	if let Some(config) = matches.value_of("action") {
        args.act = String::from(config)
	}

	if let Some(config) = matches.value_of("env") {
        args.env = String::from(config)
	}

	if let Some(config) = matches.value_of("system") {
        args.sys = String::from(config)
	}
}
