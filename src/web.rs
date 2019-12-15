use tiny_http::{ Server, Response };
use std::io::Read;

pub fn serve(plot_png_data: Vec<u8>) {
    let server = Server::http("0.0.0.0:8080").expect("Failed to create HTTP server");

    println!("Going into server loop");
    loop {
        let request = server.recv().expect("Error from server.recv");
        // TODO I don't want to copy this data
        // I think the API makes that impossible though?
        let response = Response::from_data(plot_png_data.clone());
        request.respond(response).expect("request.response failed");
    }
}
