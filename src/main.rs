#![feature(plugin)]
#![plugin(rocket_codegen)]

mod controllers;
mod models;
mod responses;

extern crate rocket;
#[macro_use]
extern crate serde_derive;

fn main() {
    rocket::ignite()
        .catch(catchers![responses::catchers::not_found])
        .mount(
            "/user",
            routes![controllers::user::all_users, controllers::user::user_by_id],
        )
        .mount("/expense", routes![controllers::expense::all_expenses])
        .launch();
}
