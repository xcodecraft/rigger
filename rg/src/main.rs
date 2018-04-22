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
//		.setting(AppSettings::SubcommandRequiredElseHelp)
		.about("rg is rigger simple name")
		.subcommands(vec![
			SubCommand::with_name("conf").about("conf project"),
			SubCommand::with_name("start").about("start project"),
			SubCommand::with_name("stop").about("stop project"),
			SubCommand::with_name("clean").about("clean project"),
			SubCommand::with_name("rc").about("rc project")])
		.arg(Arg::with_name("env")
			.short("e")
			.takes_value(true)
			.requires("system")
			.help("env"))
		.arg(Arg::with_name("system")
			.short("s")
			.takes_value(true)
			.help("system"))
		.get_matches();


	match matches.subcommand_name() {
		Some("conf") => println!("subcmd conf"),
		Some("start") => println!("subcmd start"),
		Some("stop") => println!("subcmd stop"),
		Some("clean") => println!("subcmd clean"),
		Some("rc") => println!("subcmd rc"),
		None => println!("No subcommand was used"),
		_ => println!("Some other subcommand was used"),
	}

	if let Some(config) = matches.value_of("env") {
		println!("A env is: {}", config);
	}

	if let Some(config) = matches.value_of("system") {
		println!("A system is: {}", config);
	}
}
