use tiny_http::{ Server, Response };

pub fn serve() {
    let server = Server::http("0.0.0.0:8080").expect("Failed to create HTTP server");

    println!("Going into server loop");
    loop {
        let request = server.recv().expect("Error from server.recv");
        let response = Response::from_string("Hello world");
        request.respond(response).expect("request.response failed");
    }
}
