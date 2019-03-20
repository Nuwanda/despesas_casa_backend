table! {
    expense_user (id) {
        id -> Integer,
        expense_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    expenses (id) {
        id -> Integer,
        payed_by -> Integer,
        amount -> BigInt,
        date -> BigInt,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(expense_user -> expenses (expense_id));
joinable!(expense_user -> users (user_id));
joinable!(expenses -> users (payed_by));

allow_tables_to_appear_in_same_query!(expense_user, expenses, users,);
