use diesel::prelude::*;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::JsonError;

use crate::domain::models::{Expense, JsonResult, NewExpense};
use crate::domain::services::database::schema::expenses;
use crate::domain::services::database::Conn as DbConn;
use crate::responses::Response;

#[get("/")]
pub fn all_expenses(conn: DbConn) -> Response<Vec<Expense>> {
    match expenses::table.load::<Expense>(&conn as &diesel::SqliteConnection) {
        Ok(results) => return Response::get(results),
        Err(err) => return Response::error(500, err.to_string()),
    }
}

#[get("/<id>")]
pub fn expense_by_id(id: i32, conn: DbConn) -> Option<Response<Expense>> {
    match expenses::table
        .filter(expenses::id.eq(id))
        .first::<Expense>(&conn as &diesel::SqliteConnection)
    {
        Ok(expense) => return Some(Response::get(expense)),
        Err(_) => return None,
    }
}

#[post("/", data = "<raw_expense>")]
pub fn create_expense(raw_expense: JsonResult<NewExpense>, conn: DbConn) -> Response<&str> {
    let expense = match raw_expense {
        Ok(expense) => expense.into_inner(),
        Err(JsonError::Io(err)) => return Response::error(500, err.to_string()),
        Err(JsonError::Parse(_data, err)) => return Response::error(422, err.to_string()),
    };

    match diesel::insert_into(expenses::table)
        .values(&expense)
        .execute(&conn as &diesel::SqliteConnection)
    {
        Ok(_) => return Response::post("Ok"),
        Err(err) => return Response::error(500, err.to_string()),
    }
}
