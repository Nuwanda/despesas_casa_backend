use rocket_contrib::Json;
use crate::models::User;

fn mock_users() -> Vec<User> {
    vec![
        User { id: String::from(":pedro"), name: String::from("Pedro") },
        User { id: String::from(":silane"), name: String::from("Silane") },
        User { id: String::from(":maia"), name: String::from("Maia") },
    ]
}

#[get("/")]
pub fn all_users() -> Json<Vec<User>> {
    Json(mock_users())
}

#[get("/<id>")]
pub fn user_by_id(id: String) -> Option<Json<User>> {
    match mock_users().into_iter().find(|u| u.id == id) {
        Some(user) => Some(Json(user)),
        None => None
    }
}

