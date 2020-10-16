use crate::error::RrbgError;
use tiny_http::{Response, Method};
use rustc_serialize::json::Json;
use crate::controller::controller::ControllerInstance;

pub async fn start() -> Result<(), RrbgError> {

    ControllerInstance::reset();
    let server = tiny_http::Server::http("0.0.0.0:8090").unwrap();
    for mut request in server.incoming_requests() {
        match request.method() {
            Method::Delete => ControllerInstance::reset(),
            Method::Post | Method::Patch => {
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).unwrap();
                let json: Json = content.parse().unwrap();
                if request.method() == &Method::Post {
                    ControllerInstance::reset();
                }
                ControllerInstance::set_array(json);
            },
            _ => {}
        };
        let response = Response::from_string("OK");
        request.respond(response);
    }
    Ok(())

}
