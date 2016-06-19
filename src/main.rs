extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/",hello_world);

    router.get("/about", about_page);

    fn about_page(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "This will be about me")))
    }

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    Iron::new(router).http("localhost:8080").unwrap();
}
