#![allow(proc_macro_derive_resolution_fallback)]

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}
