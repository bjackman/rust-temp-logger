use crate::plot;

use tiny_http::{ Server, Response, Header };
use crate::db::TempDb;
use plot::plot_png;
use std::io::Cursor;

fn gen_response(db: &mut TempDb) -> Response<Cursor<Vec<u8>>> {
    match plot_png(db) {
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
            Err(e) => {
                error!("Internal plotting error: {}", e);
                Response::from_string("Internal server error")
                    .with_status_code(500)
            }
        }
}

pub fn serve(db: &mut TempDb) {
    let server = Server::http("0.0.0.0:8080").expect("Failed to create HTTP server");

    info!("Going into server loop");
    loop {
        let request = server.recv().expect("Error from server.recv");
        let response = gen_response(db);
        request.respond(response).expect("request.response failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{ Connection, };
    use std::time::{ UNIX_EPOCH, Duration };
    use crate::db::{ Temp, degree_celsius };
    use tiny_http::HTTPVersion;

    // The tiny_http module doesn't expose any information about response
    // objects so our only choice for testing is to dump it and compare the
    // strings!

    #[test]
    fn returns_200() {
        let conn = Connection::open_in_memory().unwrap();
        let mut db = TempDb::new(&conn).unwrap();
        db.insert(UNIX_EPOCH + Duration::from_secs(1000),
                  Temp::new::<degree_celsius>(30.)).unwrap();
        let response = gen_response(&mut db);
        let mut response_buf = Vec::new();
        response.raw_print(Cursor::new(&mut response_buf), HTTPVersion(1, 1), &[], false, None).unwrap();
        let response_header = String::from_utf8(response_buf[..64].to_vec()).unwrap();
        assert_eq!(&response_header[..12], "HTTP/1.1 200");
    }

    #[test]
    fn returns_503() {
        let conn = Connection::open_in_memory().unwrap();
        let mut db = TempDb::new(&conn).unwrap();
        let response = gen_response(&mut db);
        let mut response_buf = Vec::new();
        response.raw_print(Cursor::new(&mut response_buf), HTTPVersion(1, 1), &[], false, None).unwrap();
        let response_header = String::from_utf8(response_buf[..64].to_vec()).unwrap();
        assert_eq!(&response_header[..12], "HTTP/1.1 503");
    }
}
