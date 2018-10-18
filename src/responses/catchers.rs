use crate::responses::Response;
use rocket::request::Request;

#[catch(400)]
pub fn bad_request(req: &Request) -> Response<String> {
    print!("{}", req);
    Response::error(400, String::from("Invalid entity"))
}

#[catch(404)]
pub fn not_found(_req: &Request) -> Response<String> {
    Response::error(404, String::from("Entity not found"))
}

#[catch(500)]
pub fn internal_error(_req: &Request) -> Response<String> {
    Response::error(500, String::from("Internal error"))
}
