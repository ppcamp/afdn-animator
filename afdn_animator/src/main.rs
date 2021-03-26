#![allow(dead_code, unused_variables)]
mod afdn;
mod util;

#[macro_use]
extern crate log;
use std;
use util::file;

fn main() {
    // configure loggers
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    // parse args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Você deve passar um arquivo de entrada");
    }
    let filename: &String = &args[1];
    debug!("Filename: {:#?}", filename);

    // parsed informations
    let infos = file::parse(filename);
    debug!("{:#?}", &infos);

    if afdn::run(&infos) {
        // caso tenha percorrido a palavra e, esta, possa ser representada pelo afd
        println!("Sucesso !!!");
    } else {
        println!("Erro :(");
    }
}
