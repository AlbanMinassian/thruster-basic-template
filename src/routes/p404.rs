use thruster::{BasicContext as Ctx, MiddlewareChain, MiddlewareReturnValue};
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
  context.body = layouts::default::render(views::p404::render());
  context.status(404);

  // -------------------------------------------------------------------
  // send
  // -------------------------------------------------------------------
  Box::new(future::ok(context))

}