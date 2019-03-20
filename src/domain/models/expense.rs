#![allow(proc_macro_derive_resolution_fallback)]

use crate::domain::services::database::schema::expenses;

// Used for db retrievals
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Expense {
    pub id: i32,
    pub payed_by: i32,
    pub amount: i64,
    pub date: i64,
}

// Used for db inserts
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "expenses"]
pub struct InnerExpense {
    pub payed_by: i32,
    pub amount: i64,
    pub date: i64,
}

// Received over the wire
#[derive(Debug, Deserialize)]
pub struct NewExpense {
    pub split_between: Vec<i32>,
    pub expense: InnerExpense,
}
