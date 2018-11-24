use crate::models::{Expense, JsonResult};
use crate::responses::Response;
use crate::DB;
use rocket::State;
use rocket_contrib::json::JsonError;

#[get("/")]
pub fn all_expenses(db: State<DB>) -> Response<Vec<Expense>> {
    Response::get(db.expenses.lock().unwrap().clone())
}

#[post("/", data = "<raw_expense>")]
pub fn create_expense(raw_expense: JsonResult<Expense>, db: State<DB>) -> Response<Expense> {
    let expense = match raw_expense {
        Ok(expense) => expense.into_inner(),
        Err(JsonError::Io(err)) => return Response::error(500, err.to_string()),
        Err(JsonError::Parse(_data, err)) => return Response::error(422, err.to_string()),
    };

    if expense.from == expense.to {
        return Response::error(
            400,
            String::from("Cannot create an expense between a user and himself"),
        );
    }

    let mut expenses = db.expenses.lock().unwrap();

    for e in expenses.iter_mut() {
        if e.is_earlier_version(&expense) {
            e.update_from_other(&expense);
        }
    }

    Response::post(expense)
}
