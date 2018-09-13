extern crate thruster;
extern crate futures;
extern crate time;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod routes;
mod layouts;
mod views;
pub mod app; // ``pub`` to allow tests

use std::error::Error;

#[cfg_attr(tarpaulin, skip)]
pub fn server(_protocol: &str, host: &str, port: &u16) -> Result<bool, Box<Error>> {

    // -------------------------------------------------------------------
    // init stdout log - usage ``export RUST_LOG=myapp=info``
    // -------------------------------------------------------------------
    pretty_env_logger::init();

    // -------------------------------------------------------------------
    // app, middleware, routes ...
    // -------------------------------------------------------------------
    let app = app::init();

    // -------------------------------------------------------------------
    // start server
    // -------------------------------------------------------------------
    thruster::App::start(app, host, *port);

    // -------------------------------------------------------------------
    // return
    // -------------------------------------------------------------------
    Ok(true)
}
