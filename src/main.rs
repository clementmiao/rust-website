#[macro_use] extern crate nickel;

use std::process::Command;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::str;
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

    let host_port = 8080; //Note: port must be 8080 for GAE
    let hostname_cmd = Command::new("hostname").arg("-I").output();
    let host_addr: SocketAddrV4 = match hostname_cmd {
        Ok(res) => {
            let addr = str::from_utf8(res.stdout.as_slice())
                .map_err(|err| err.to_string())
                .and_then(|ip_str| ip_str.trim()
                            .parse::<Ipv4Addr>()
                            .map_err(|err| err.to_string()))
                .map(|ip| SocketAddrV4::new(ip, host_port));

            match addr {
                Ok(addr) => addr,
                Err(_) => {
                    let ip = Ipv4Addr::new(127, 0, 0, 1);
                    SocketAddrV4::new(ip, host_port)
                }
            }
        },
        Err(_) => {
            let ip = Ipv4Addr::new(127, 0, 0, 1);
            SocketAddrV4::new(ip, host_port)
        }
    };
    println!("Server listening at {}", host_addr);

    server.listen(host_addr);
}
