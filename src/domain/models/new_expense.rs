use crate::domain::services::database::schema::expenses;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "expenses"]
pub struct NewExpense {
    pub payed_by: i32,
    pub amount: i64,
    pub date: i64,
}
