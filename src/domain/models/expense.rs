#![allow(proc_macro_derive_resolution_fallback)]

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Expense {
    pub id: i32,
    pub payed_by: i32,
    pub amount: i64,
    pub date: i64,
}
