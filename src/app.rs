use thruster::{App, BasicContext as Ctx, Request};

use routes;

pub fn init() -> App<Request, Ctx>  {

    // -------------------------------------------------------------------
    // app
    // -------------------------------------------------------------------
    let mut app = App::<Request, Ctx>::new();

    // -------------------------------------------------------------------
    // add middlewares
    // -------------------------------------------------------------------
    app.get("/", vec![routes::root::middleware]);
    app.get("/index.html", vec![routes::index::middleware]);
    app.get("/example/text", vec![routes::plaintext::middleware]);
    app.get("/example/json", vec![routes::basicjson::middleware]);
    app.get("/*", vec![routes::p404::middleware]);

    // -------------------------------------------------------------------
    // return
    // -------------------------------------------------------------------
    app

}
