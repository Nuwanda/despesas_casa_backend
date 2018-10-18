use crate::models::Expense;
use crate::responses::Response;
use rocket_contrib::Json;
use serde_json::Error as SerdeError;
use std::error::Error;

fn mock_expenses() -> Vec<Expense> {
    vec![
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
    ]
}

#[get("/")]
pub fn all_expenses() -> Response<Vec<Expense>> {
    let data = mock_expenses();
    Response::get(data)
}

#[post("/", data = "<expense>")]
pub fn create_expense(expense: Result<Json<Expense>, SerdeError>) -> Response<Expense> {
    match expense {
        Ok(expense) => Response::post(expense.into_inner()),
        Err(ref error) => Response::error(400, format!("{}", error)),
    }
}
