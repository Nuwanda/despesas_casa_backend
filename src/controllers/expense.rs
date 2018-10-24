use crate::models::{Expense, SerdeResult};
use crate::responses::Response;
use crate::DB;
use rocket::State;

#[get("/")]
pub fn all_expenses(db: State<DB>) -> Response<Vec<Expense>> {
    Response::get(db.expenses.lock().unwrap().clone())
}

#[post("/", data = "<raw_expense>")]
pub fn create_expense(raw_expense: SerdeResult<Expense>, db: State<DB>) -> Response<Expense> {
    let expense = match raw_expense {
        Ok(expense) => expense.into_inner(),
        Err(ref error) => return Response::error(400, format!("{}", error)),
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
