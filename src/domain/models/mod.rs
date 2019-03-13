use rocket_contrib::json::{Json, JsonError};

pub use self::expense::Expense;
pub use self::expense_user::ExpenseUser;
pub use self::new_expense::NewExpense;
pub use self::user::User;

mod expense;
mod expense_user;
mod new_expense;
mod user;

pub type JsonResult<'a, T> = Result<Json<T>, JsonError<'a>>;
