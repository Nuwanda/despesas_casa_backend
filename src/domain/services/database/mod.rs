use rocket_contrib::databases::diesel;

pub mod schema;

#[database("main_db")]
pub struct Conn(diesel::SqliteConnection);
