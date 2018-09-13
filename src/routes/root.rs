use thruster::{BasicContext as Ctx, MiddlewareChain, MiddlewareReturnValue};
use std::boxed::Box;
use futures::future;

pub fn middleware(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {

  // -------------------------------------------------------------------
  // stdout
  // -------------------------------------------------------------------
  info!("request: {}, {}", "?method?", "?path?");
  trace!("{}#{}", file!(), line!());

  // -------------------------------------------------------------------
  // context
  // -------------------------------------------------------------------
  info!("redirect to /index.html");
  context.redirect("/index.html");

  // -------------------------------------------------------------------
  // send
  // -------------------------------------------------------------------
  Box::new(future::ok(context))

}