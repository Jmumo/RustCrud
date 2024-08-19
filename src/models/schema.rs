// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        location -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
