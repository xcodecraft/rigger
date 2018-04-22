
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate rg_core ;


fn main() {

    pretty_env_logger::init();
    rg_core::rg_main();
}
