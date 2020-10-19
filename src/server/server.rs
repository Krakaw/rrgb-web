use crate::controller::controller::ControllerInstance;
use crate::error::RrbgError;
use rustc_serialize::json::{self};

use std::collections::HashMap;
use std::env;
use tiny_http::{Method, Response};

#[derive(RustcDecodable, Default, Debug)]
pub struct LedValueRequest {
    values: HashMap<usize, [u8; 4]>,
}

pub async fn start() -> Result<(), RrbgError> {
    ControllerInstance::reset();
    let server_addr = env::var("LISTEN_ADDRESS").unwrap_or("0.0.0.0:8090".to_string());
    eprintln!("server_addr = {:?}", server_addr);
    let server = tiny_http::Server::http(server_addr).unwrap();
    let mut current_led_values: HashMap<usize, [u8; 4]> = HashMap::new();
    for mut request in server.incoming_requests() {
        match request.method() {
            Method::Delete => {
                current_led_values = HashMap::new();
                ControllerInstance::reset();
            }
            Method::Get | Method::Post | Method::Patch => {
                let url = request.url().chars().skip(1).collect::<String>();
                let query_params = url.split("/").collect::<Vec<&str>>();
                let request_led_values: LedValueRequest = if query_params.len() > 1 {
                    let mut values = HashMap::new();
                    values.insert(
                        query_params
                            .get(0)
                            .map(|v| v.parse::<usize>().unwrap_or(0))
                            .unwrap_or(0),
                        [
                            query_params
                                .get(1)
                                .map(|v| v.parse::<u8>().unwrap_or(0))
                                .unwrap_or(0),
                            query_params
                                .get(2)
                                .map(|v| v.parse::<u8>().unwrap_or(0))
                                .unwrap_or(0),
                            query_params
                                .get(3)
                                .map(|v| v.parse::<u8>().unwrap_or(0))
                                .unwrap_or(0),
                            query_params
                                .get(4)
                                .map(|v| v.parse::<u8>().unwrap_or(0))
                                .unwrap_or(0),
                        ],
                    );
                    LedValueRequest { values }
                } else {
                    let mut content = String::new();
                    request.as_reader().read_to_string(&mut content).unwrap();
                    json::decode(&content).unwrap_or(LedValueRequest {
                        ..LedValueRequest::default()
                    })
                };

                if request.method() == &Method::Post {
                    current_led_values = HashMap::new();
                }
                current_led_values =
                    merge_arrays(current_led_values.clone(), request_led_values.values);

                ControllerInstance::set_array(current_led_values.clone());
            }
            _ => {}
        };
        let response = Response::from_string("OK");
        request.respond(response);
    }
    Ok(())
}

fn merge_arrays(
    mut current_values: HashMap<usize, [u8; 4]>,
    request_led_values: HashMap<usize, [u8; 4]>,
) -> HashMap<usize, [u8; 4]> {
    current_values.extend(request_led_values.into_iter());
    current_values
}
