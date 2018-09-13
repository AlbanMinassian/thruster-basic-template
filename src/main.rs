extern crate clap;
extern crate colored;
use colored::*;
extern crate myapplib;

// -------------------------------------------------------------------
// main
// -------------------------------------------------------------------
#[cfg_attr(tarpaulin, skip)]
fn main() {

  // -------------------------------------------------------------------
  // match args
  // -------------------------------------------------------------------
  let default_port = "4321";
  let matches = clap::App::new("{{project-name}}")
                        .version("1.0")
                        .author("{{authors}}")
                        .about("PROJECT_DESCRIPTION_INLINE")
                        .arg(clap::Arg::with_name("port")
                              .short("p")
                              .long("port")
                              .env("PORT")
                              .value_name("PORT")
                              .help(format!("set port, default={}", &default_port).as_str())
                              .takes_value(true))
                        .get_matches();

  // -------------------------------------------------------------------
  // port, host ...
  // -------------------------------------------------------------------
  let protocol= "http";
  let host= "127.0.0.1";
  let port= matches.value_of("port").unwrap_or(&default_port).parse::<u16>().unwrap();

  // -------------------------------------------------------------------
  // start lib server
  // -------------------------------------------------------------------
  println!(" {}  {} > Starting server {}://{}:{}", "INFO".green(), "{{project-name}}".bright_white().bold(), &protocol, &host, &port);
  let _result = myapplib::server(&protocol, &host, &port);

}