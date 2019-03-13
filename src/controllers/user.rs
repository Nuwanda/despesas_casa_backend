use diesel::prelude::*;
use rocket_contrib::databases::diesel;

use crate::domain::models::User;
use crate::domain::services::database::schema::users;
use crate::domain::services::database::Conn as DbConn;
use crate::responses::Response;

#[get("/")]
pub fn all_users(conn: DbConn) -> Response<Vec<User>> {
    match users::table.load::<User>(&conn as &diesel::SqliteConnection) {
        Ok(results) => return Response::get(results),
        Err(err) => return Response::error(500, err.to_string()),
    }
}

#[get("/<id>")]
pub fn user_by_id(id: i32, conn: DbConn) -> Option<Response<User>> {
    match users::table
        .filter(users::id.eq(id))
        .first::<User>(&conn as &diesel::SqliteConnection)
    {
        Ok(user) => return Some(Response::get(user)),
        Err(_) => return None,
    }
}
