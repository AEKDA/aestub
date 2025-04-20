use std::error::Error;

mod parser;

#[derive(Debug)]
pub struct Route {
    pub method: Method,
    pub status_code: u16,
    pub path: String,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
}

impl Route {
    pub fn from(input: &str) -> Result<Vec<Route>, Box<dyn Error>> {
        let routes = parser::parse_routes(input);
        match routes {
            Ok(r) => Ok(r.1),
            Err(e) => {
                println!("{}", e);
                todo!()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Option,
    Delete,
    Head,
    Trace,
    Connect,
}

impl Method {
    pub fn from(input: &str) -> Option<Method> {
        match input.to_uppercase().as_str() {
            "GET" => Some(Method::Get),
            "POST" => Some(Method::Post),
            "PUT" => Some(Method::Put),
            "PATCH" => Some(Method::Patch),
            "DELETE" => Some(Method::Delete),
            "HEAD" => Some(Method::Head),
            "TRACE" => Some(Method::Trace),
            "OPTION" => Some(Method::Option),
            "CONNECT" => Some(Method::Connect),
            _ => None,
        }
    }
}
