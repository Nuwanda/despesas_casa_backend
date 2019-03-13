#![allow(proc_macro_derive_resolution_fallback)]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct ExpenseUser {
    pub expense_id: i32,
    pub user_id: i32,
}
