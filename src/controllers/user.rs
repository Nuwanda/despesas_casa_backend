use crate::models::User;
use crate::responses::Response;

fn mock_users() -> Vec<User> {
    vec![
        User {
            id: String::from(":pedro"),
            name: String::from("Pedro"),
        },
        User {
            id: String::from(":silane"),
            name: String::from("Silane"),
        },
        User {
            id: String::from(":maia"),
            name: String::from("Maia"),
        },
    ]
}

#[get("/")]
pub fn all_users() -> Response<Vec<User>> {
    Response::get(mock_users())
}

#[get("/<id>")]
pub fn user_by_id(id: String) -> Option<Response<User>> {
    match mock_users().into_iter().find(|u| u.id == id) {
        Some(user) => Some(Response::get(user)),
        None => None,
    }
}
