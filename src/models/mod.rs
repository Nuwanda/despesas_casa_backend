pub use self::expense::Expense;
pub use self::user::User;
use rocket_contrib::{Json, JsonError};

mod expense;
mod user;

pub type JsonResult<T> = Result<Json<T>, JsonError>;
