use std::io::prelude::*;

use std::str;

use std::net::TcpListener;
use std::net::TcpStream;

#[macro_use]
extern crate serde_json;
extern crate handlebars;
use handlebars::Handlebars;


mod strings;

pub fn setup_templates() -> Handlebars {
    let mut reg = Handlebars::new();
    let template_string = strings::get_file("templates/template.html");

    // UNWRAP?
    reg.register_template_string("tmpl", template_string).unwrap();
    reg    
}

pub fn start(port: &str, template_engine: &Handlebars) {

    // UNWRAP?
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap(), template_engine)
    }    
}

fn handle_connection(mut stream: TcpStream, reg: &Handlebars) {

	let mut buffer = [0; 512];

    // UNWRAP?
	stream.read(&mut buffer).unwrap();

    let (method, route) = parse_request(&buffer);

    let data = &json!({
        "route": route
    });

    let rendered_template = reg.render("tmpl", data ).unwrap();

    let (header, content) = match method {
        "GET" =>  ("HTTP/1.1 200 OK\r\n\r\n", rendered_template),
        _ => ("HTTP/1.1 500\r\n\r\n", strings::get_file("templates/500.html"))
    };

    let response = format!("{}{}", header, content);

    respond(stream, response);
}

fn respond(mut stream: TcpStream, response: String) {

    // UNWRAP?
    stream.write(response.as_bytes()).unwrap();

    // UNWRAP?
    stream.flush().unwrap();
}

fn get_route(request: &str) -> &str {
    strings::nth_word(request, 2)
}

fn get_method(request: &str) -> &str {
    strings::nth_word(request, 1)
}

fn parse_request(request: &[u8; 512]) -> (&str, &str) {

    // UNWRAP?
    let request = str::from_utf8(request).unwrap();
    (get_method(request), get_route(request))

}

