#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: i64,
}
