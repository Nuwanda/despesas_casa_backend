use rocket::request::Request;
use rocket::response::{Responder, Result};
use rocket_contrib::Json;
use serde::Serialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::new(5, 0))
        .as_secs()
}

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub time: u64,
}

impl<T: Serialize> Response<T> {
    fn new(data: T) -> Response<T> {
        Response {
            status: 200,
            data: Some(data),
            error_message: None,
            time: now(),
        }
    }

    pub fn get(data: T) -> Response<T> {
        Response::new(data)
    }

    pub fn post(data: T) -> Response<T> {
        Response {
            status: 201,
            data: Some(data),
            error_message: None,
            time: now(),
        }
    }

    pub fn error(status: i32, message: String) -> Response<T> {
        Response {
            status,
            data: None,
            error_message: Some(message),
            time: now(),
        }
    }
}

impl<T: Serialize> Responder<'static> for Response<T> {
    fn respond_to(self, req: &Request) -> Result<'static> {
        Json(self).respond_to(req)
    }
}
