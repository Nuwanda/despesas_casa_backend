use crate::models::Expense;
use crate::responses::Response;
use crate::DB;
use rocket::State;
use rocket_contrib::Json;
use serde_json::Error as SerdeError;

#[get("/")]
pub fn all_expenses(db: State<DB>) -> Response<Vec<Expense>> {
    Response::get(db.expenses.clone())
}

#[post("/", data = "<raw_expense>")]
pub fn create_expense(
    raw_expense: Result<Json<Expense>, SerdeError>,
    db: State<DB>,
) -> Response<Expense> {
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

    let expenses: Vec<Expense> = db
        .expenses
        .iter()
        .map(|e| {
            if e.is_earlier_version(&expense) {
                e.update_from_other(&expense)
            } else {
                e.clone()
            }
        })
        .collect();

    print!("{:#?}", expenses);

    Response::post(expense)
}
