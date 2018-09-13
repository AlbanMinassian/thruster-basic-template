use thruster::{BasicContext as Ctx, App};

use routes;

pub fn init() -> App<Ctx>  {

    // -------------------------------------------------------------------
    // app
    // -------------------------------------------------------------------
    let mut app = App::<Ctx>::new();

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