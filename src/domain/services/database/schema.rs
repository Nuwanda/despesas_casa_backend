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

joinable!(expenses -> users (payed_by));

allow_tables_to_appear_in_same_query!(expenses, users,);
