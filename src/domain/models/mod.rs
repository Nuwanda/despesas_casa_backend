use rocket_contrib::json::{Json, JsonError};

pub use self::expense::{Expense, NewExpense};
pub use self::expense_user::{ExpenseUser, NewExpenseUser};
pub use self::user::User;

mod expense;
mod expense_user;
mod user;

pub type JsonResult<'a, T> = Result<Json<T>, JsonError<'a>>;
