// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate env_logger;
extern crate actix_web;
extern crate postgres;
extern crate oracle;

extern crate serde;
extern crate serde_json;

extern crate inner;
extern crate futures;
extern crate csv;
extern crate ini;
extern crate bio;
extern crate rayon;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;


#[macro_use]
extern crate error_chain;

mod controller;
mod router;
mod database;
mod models;
mod errors;
mod misc;
mod flatten;
mod query_builder;
mod msa;

use actix_web::middleware::Logger;
use actix_web::*;


pub use errors::*;
use ini::Ini;

pub struct State {
      pub db_params: database::DBParams,
}

fn main() {
    //Logging is hard coded for now, but soon can be configured through config file    
    std::env::set_var("RUST_LOG", "actix_web=info,iptmnet_api=info");
    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .default_format_module_path(false)
        .init();

    let conf;
    let conf_result = Ini::load_from_file("config.ini");
    match conf_result {
            Ok(value) => {
                conf = value;
            },
            Err(error) => {
                error!("{}",error);
                std::process::exit(1);
            }
    }

    let db_params = misc::parse_configs(&conf);    

    let app = move || {
            let app = App::with_state(State{db_params: db_params.clone()})
                                  .middleware(Logger::new("STATUS : %s | %t | %D ms | PID: %P | %r "));
            return router::init_routes(app);    
    };

    server::HttpServer::new(app)
            .bind("0.0.0.0:8088")
            .expect("Can not bind to 0.0.0.0:8080")
            .keep_alive(None)
            .run();    
}
