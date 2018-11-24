#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use self::models::{Expense, User};
// TODO: Just for testing, remove later when we connect to a DB
use std::sync::Mutex;

mod controllers;
mod models;
mod responses;

pub struct DB {
    users: Mutex<Vec<User>>,
    expenses: Mutex<Vec<Expense>>,
}

fn main() {
    rocket::ignite()
        .register(catchers![
            responses::catchers::bad_request,
            responses::catchers::not_found,
            responses::catchers::internal_error
        ])
        .mount(
            "/user",
            routes![controllers::user::all_users, controllers::user::user_by_id],
        )
        .mount(
            "/expense",
            routes![
                controllers::expense::all_expenses,
                controllers::expense::create_expense
            ],
        )
        .manage(DB {
            users: Mutex::new(vec![
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
            ]),
            expenses: Mutex::new(vec![
                Expense {
                    id: String::from(":silane_pedro"),
                    from: String::from(":silane"),
                    to: String::from(":pedro"),
                    amount: 38640,
                },
                Expense {
                    id: String::from(":maia_pedro"),
                    from: String::from(":maia"),
                    to: String::from(":pedro"),
                    amount: 40000,
                },
            ]),
        })
        .launch();
}
