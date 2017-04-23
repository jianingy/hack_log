#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate hack_log;

use clap::App;

fn main () {
    let opt_config = load_yaml!("options.yaml");
    let opts = App::from_yaml(opt_config).get_matches();
    hack_log::init(Some(&opts), None).unwrap();
    error!("Hello, world!");
}
