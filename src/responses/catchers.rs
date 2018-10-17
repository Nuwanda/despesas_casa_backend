use crate::responses::Response;
use rocket::request::Request;

#[catch(404)]
pub fn not_found(_req: &Request) -> Response<String> {
    Response::error(404, String::from("Entity not found"))
}
