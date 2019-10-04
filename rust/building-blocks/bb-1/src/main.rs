#![deny(missing_docs)]
//! a simple example for clap
use clap::{App, Arg};

fn main() {
    let cli_args_matches = App::new("bb-1-test")
        .version("1.0")
        .author("flame4 <646361781@qq.com>")
        .about("clap test")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("config")
            .help("config file, json")
            .takes_value(true))
        .get_matches();

    let config_file = cli_args_matches.value_of("config").unwrap();
    println!("config file = {:?}", config_file);
}
