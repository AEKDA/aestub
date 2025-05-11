use nom::{
    IResult, Parser,
    bytes::{
        complete::{take_till, take_until},
        tag,
    },
    character::complete::{digit1, line_ending, not_line_ending, space1},
    combinator::opt,
    error::{Error, ErrorKind},
    multi::{many0, many1},
    sequence::terminated,
};

use super::{Method, Route};

fn parse_method(input: &str) -> IResult<&str, Method> {
    let (input, method) = take_till(|c| c == ' ').parse(input)?;
    match Method::from(method) {
        Some(m) => Ok((input, m)),
        None => Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
    }
}
fn parse_http_code(input: &str) -> IResult<&str, u16> {
    let (input, status_code) = digit1(input)?;

    let status_code = u16::from_str_radix(status_code, 10);
    match status_code {
        Ok(code) => Ok((input, code)),
        Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::Digit))),
    }
}
fn parse_response_body(input: &str) -> IResult<&str, Option<String>> {
    let (input, response_body) = take_until("EOF").parse(input)?;
    let (input, _) = tag("EOF").parse(input)?;
    match response_body {
        "" => Ok((input, None)),
        _ => Ok((input, Some(response_body.to_string()))),
    }
}
fn parse_path(input: &str) -> IResult<&str, String> {
    let (input, path) = not_line_ending(input)?;
    Ok((input, path.to_string()))
}
fn parse_route(input: &str) -> IResult<&str, Route> {
    let request_body = Some("".to_string());
    let (input, (method, _, status_code, _, path, _, response_body)) = ((
        parse_method,
        space1,
        parse_http_code,
        space1,
        parse_path,
        line_ending,
        parse_response_body,
    ))
        .parse(input)?;

    Ok((
        input,
        Route {
            method,
            status_code,
            _request_body: request_body,
            response_body,
            path,
        },
    ))
}
pub fn parse_routes(input: &str) -> IResult<&str, Vec<Route>> {
    many1(terminated(parse_route, opt(many0(line_ending)))).parse(input)
}
