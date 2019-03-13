#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use self::domain::services::database::Conn as DbConn;

mod controllers;
mod domain;
mod responses;

fn main() {
    rocket::ignite()
        .register(catchers![
            responses::catchers::bad_request,
            responses::catchers::not_found,
            responses::catchers::internal_error
        ])
        .attach(DbConn::fairing())
        .mount(
            "/user",
            routes![controllers::user::all_users, controllers::user::user_by_id],
        )
        .mount(
            "/expense",
            routes![
                controllers::expense::all_expenses,
                controllers::expense::create_expense,
                controllers::expense::expense_by_id
            ],
        )
        .launch();
}
