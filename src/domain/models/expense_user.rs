#![allow(proc_macro_derive_resolution_fallback)]

use crate::domain::services::database::schema::expense_user;

// Used for db retrievals
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct ExpenseUser {
    pub id: i32,
    pub expense_id: i32,
    pub user_id: i32,
}

// Used for db inserts
#[derive(Debug, Insertable)]
#[table_name = "expense_user"]
pub struct NewExpenseUser {
    pub expense_id: i32,
    pub user_id: i32,
}
