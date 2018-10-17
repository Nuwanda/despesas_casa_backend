use crate::models::Expense;
use crate::responses::Response;

fn mock_expenses() -> Vec<Expense> {
    vec![
        Expense {
            id: String::from(":silane_pedro"),
            from: String::from(":silane"),
            to: String::from(":pedro"),
            amount: 38640,
        },
        Expense {
            id: String::from(":maia_pedro"),
            from: String::from(":maia"),
            to: String::from(":pedro"),
            amount: 40000,
        },
    ]
}

#[get("/")]
pub fn all_expenses() -> Response<Vec<Expense>> {
    let data = mock_expenses();
    Response::get(data)
}
