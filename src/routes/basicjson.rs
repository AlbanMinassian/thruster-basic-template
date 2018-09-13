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
  context.set("Content-Type", "application/json");
  context.body = "{\"Hello\":\"Json !\"}".to_owned();

  // -------------------------------------------------------------------
  // send
  // -------------------------------------------------------------------
  Box::new(future::ok(context))

}