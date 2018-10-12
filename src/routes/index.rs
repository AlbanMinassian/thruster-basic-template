use thruster::{BasicContext as Ctx, CookieOptions, MiddlewareChain, MiddlewareReturnValue};
use std::boxed::Box;
use futures::future;

use layouts;
use views;

pub fn middleware(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {

  // -------------------------------------------------------------------
  // stdout
  // -------------------------------------------------------------------
  info!("request: {}, {}", "?method?", "?path?");
  trace!("{}#{}", file!(), line!());

  // -------------------------------------------------------------------
  // context
  // -------------------------------------------------------------------
  context.set("Content-Type", "text/html");
  context.body = layouts::default::render(views::index::render());
  context.cookie("SomeCookie", "Some Value!", &CookieOptions::default());

  // -------------------------------------------------------------------
  // send
  // -------------------------------------------------------------------
  Box::new(future::ok(context))

}
