mod expense;
mod user;

pub use self::expense::Expense;
pub use self::user::User;

use rocket_contrib::Json;
use serde_json::Error as SerdeError;

pub type SerdeResult<T> = Result<Json<T>, SerdeError>;
