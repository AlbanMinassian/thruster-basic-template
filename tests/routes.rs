extern crate myapplib;
extern crate thruster;
use thruster::{testing};

#[test]
fn test_route_example_textplain() {

    let app = myapplib::app::init();
    let result = testing::get(&app, "/example/text");
    assert!(result.body == "Hello Text !");

}

#[test]
fn test_route_example_basicjson() {

    let app = myapplib::app::init();
    let result = testing::get(&app, "/example/json");
    assert!(result.body == "{\"Hello\":\"Json !\"}");

}

#[test]
fn test_route_root_redirect() {

    let app = myapplib::app::init();
    let _result = testing::get(&app, "/");

}

#[test]
fn test_route_index() {

    let app = myapplib::app::init();
    let _result = testing::get(&app, "/index.html");

}

#[test]
fn test_route_notexist() {

    let app = myapplib::app::init();
    let _result = testing::get(&app, "/notexist");

}
