use crate::plot;

use tiny_http::{ Server, Response, Header };
use crate::db::TempDb;
use plot::plot_png;

pub fn serve(db: &mut TempDb) {
    let server = Server::http("0.0.0.0:8080").expect("Failed to create HTTP server");

    println!("Going into server loop");
    loop {
        let request = server.recv().expect("Error from server.recv");
        let response = match plot_png(db) {
            Ok(png_data) =>  {
                // TODO I don't want to copy this data
                // I think the API makes that impossible though?
                Response::from_data(png_data.clone())
            },
            Err(plot::Error::NoDataError) => {
                Response::from_string("No data yet")
                    .with_status_code(503)
                    .with_header("Retry-After: 1".parse::<Header>().unwrap())
            }
            Err(_) => {
                Response::from_string("Internal server error")
                    .with_status_code(500)
            }
        };
        request.respond(response).expect("request.response failed");
    }
}
