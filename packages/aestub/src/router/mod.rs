use std::error::Error;

use axum::routing::MethodFilter;

mod parser;

#[derive(Debug, Clone)]
pub struct Route {
    pub method: Method,
    pub status_code: u16,
    pub path: String,
    pub _request_body: Option<String>,
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

#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Options,
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
            "OPTIONS" => Some(Method::Options),
            "CONNECT" => Some(Method::Connect),
            _ => None,
        }
    }

    pub fn to_method_filter(&self) -> MethodFilter {
        match self {
            Method::Get => MethodFilter::GET,
            Method::Post => MethodFilter::POST,
            Method::Put => MethodFilter::PUT,
            Method::Patch => MethodFilter::PATCH,
            Method::Options => MethodFilter::OPTIONS,
            Method::Delete => MethodFilter::DELETE,
            Method::Head => MethodFilter::HEAD,
            Method::Trace => MethodFilter::TRACE,
            Method::Connect => MethodFilter::CONNECT,
        }
    }
}
