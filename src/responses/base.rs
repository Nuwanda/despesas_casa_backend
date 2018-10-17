use rocket::request::Request;
use rocket::response::{Responder, Result};
use rocket_contrib::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub status: i32,
    pub data: Option<T>,
    pub error_message: Option<String>,
}

impl<T: Serialize> Response<T> {
    fn new(data: T) -> Response<T> {
        Response {
            status: 200,
            data: Some(data),
            error_message: None,
        }
    }

    pub fn get(data: T) -> Response<T> {
        Response::new(data)
    }

    /* pub fn put(data: T) -> Response<T> {
        Response::new(data)
    }
    
    pub fn delete(data: T) -> Response<T> {
        Response::new(data)
    }
    
    pub fn post(data: T) -> Response<T> {
        Response {
            status: 201,
            data: Some(data),
            error_message: None,
        }
    } */

    pub fn error(code: i32, message: String) -> Response<T> {
        Response {
            status: code,
            data: None,
            error_message: Some(message),
        }
    }
}

impl<T: Serialize> Responder<'static> for Response<T> {
    fn respond_to(self, req: &Request) -> Result<'static> {
        Json(self).respond_to(req)
    }
}
