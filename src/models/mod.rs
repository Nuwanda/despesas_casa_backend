pub use self::expense::Expense;
pub use self::user::User;
use rocket_contrib::json::{Json, JsonError};

mod expense;
mod user;

pub type JsonResult<'a, T> = Result<Json<T>, JsonError<'a>>;
