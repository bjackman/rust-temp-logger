use tiny_http::{ Server, Response };
use std::fs::File;
use std::io::Read;

pub fn serve() {
    let server = Server::http("0.0.0.0:8080").expect("Failed to create HTTP server");

    let mut file = File::open("/tmp/plot.png").expect("Failed to open plot file");
    let mut plot_png_data = Vec::new();
    file.read_to_end(&mut plot_png_data).expect("Failed to read PNG data from file");

    println!("Going into server loop");
    loop {
        let request = server.recv().expect("Error from server.recv");
        // TODO I don't want to copy this data
        // I think the API makes that impossible though?
        let response = Response::from_data(plot_png_data.clone());
        request.respond(response).expect("request.response failed");
    }
}
