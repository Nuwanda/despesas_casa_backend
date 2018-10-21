use crate::models::User;
use crate::responses::Response;
use crate::DB;
use rocket::State;

#[get("/")]
pub fn all_users(db: State<DB>) -> Response<Vec<User>> {
    Response::get(db.users.lock().unwrap().clone())
}

#[get("/<id>")]
pub fn user_by_id(id: String, db: State<DB>) -> Option<Response<User>> {
    match db
        .users
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|u| u.id == id)
    {
        Some(user) => Some(Response::get(user)),
        None => None,
    }
}
