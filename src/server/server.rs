use crate::controller::controller::ControllerInstance;
use crate::error::RrbgError;
use rustc_serialize::json::{self};
use std::collections::HashMap;
use std::env;
use std::io::Cursor;
use std::str::FromStr;
use tiny_http::{Method, Response};

#[derive(RustcDecodable, RustcEncodable, Default, Debug)]
pub struct LedValueRequest {
    values: HashMap<usize, LedValue>,
}

#[derive(RustcDecodable, RustcEncodable, Default, Debug, Clone)]
pub struct LedValue {
    pub name: Option<String>,
    pub rgb: [u8; 4],
}

fn respond_html(led_count: i32) -> Response<Cursor<Vec<u8>>> {
    let html = include_str!("../../public/index.html");
    let html = html.replace("__LED_COUNT__", &led_count.to_string());
    let response = Response::from_string(html).with_header(
        "Content-type: text/html"
            .parse::<tiny_http::Header>()
            .unwrap(),
    );
    response
}

fn extract_int<T>(values: &Vec<&str>, get_index: usize, default: T) -> T
where
    T: FromStr + Clone,
{
    values
        .get(get_index)
        .map(|v| v.parse::<T>().unwrap_or(default.clone()))
        .unwrap_or(default)
}

pub async fn start() -> Result<(), RrbgError> {
    let led_count = env::var("LED_COUNT")
        .unwrap_or("32".to_string())
        .parse()
        .unwrap_or(32);
    ControllerInstance::reset();
    let server_addr = env::var("LISTEN_ADDRESS").unwrap_or("0.0.0.0:8090".to_string());
    eprintln!("server_addr = {:?}", server_addr);
    let server = tiny_http::Server::http(server_addr).unwrap();
    let mut current_led_values: HashMap<usize, LedValue> = HashMap::new();
    for mut request in server.incoming_requests() {
        current_led_values = match request.method() {
            Method::Post | Method::Delete => HashMap::new(),
            _ => current_led_values,
        };
        let response = match request.url() {
            "/index.html" => respond_html(led_count),
            _ => {
                match request.method() {
                    Method::Delete => {
                        ControllerInstance::reset();
                    }
                    Method::Get | Method::Post | Method::Patch => {
                        let url = request.url();
                        if !(url == "/" && request.method() == &Method::Get) {
                            let url = url.chars().skip(1).collect::<String>();
                            let query_params = url.split("/").collect::<Vec<&str>>();
                            let request_led_values: LedValueRequest = if query_params.len() > 1 {
                                let mut values = HashMap::new();
                                values.insert(
                                    extract_int(&query_params, 0, 0),
                                    LedValue {
                                        rgb: [
                                            extract_int(&query_params, 1, 0u8),
                                            extract_int(&query_params, 2, 0u8),
                                            extract_int(&query_params, 3, 0u8),
                                            extract_int(&query_params, 4, 0u8),
                                        ],
                                        name: None,
                                    },
                                );
                                LedValueRequest { values }
                            } else {
                                let mut content = String::new();
                                request.as_reader().read_to_string(&mut content).unwrap();
                                json::decode(&content).unwrap_or(LedValueRequest {
                                    ..LedValueRequest::default()
                                })
                            };

                            current_led_values =
                                merge_arrays(current_led_values.clone(), request_led_values.values);

                            ControllerInstance::set_array(current_led_values.clone());
                        }
                    }
                    _ => {}
                };
                Response::from_string(json::encode(&current_led_values).unwrap())
            }
        };
        request.respond(response);
    }
    Ok(())
}

fn merge_arrays(
    mut current_values: HashMap<usize, LedValue>,
    request_led_values: HashMap<usize, LedValue>,
) -> HashMap<usize, LedValue> {
    current_values.extend(request_led_values.into_iter());
    current_values
}
