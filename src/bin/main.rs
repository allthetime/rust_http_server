extern crate server;

fn main() {
    server::start("8088", &server::setup_templates())
}
