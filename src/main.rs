#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter,StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("text", "Coming Soon");
        return response.render("assets/template.tpl", &data);
    });

    server.utilize(StaticFilesHandler::new("assets/css/"));

    server.listen("127.0.0.1:6767");
}
