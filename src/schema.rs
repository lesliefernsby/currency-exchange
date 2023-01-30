// @generated automatically by Diesel CLI.

diesel::table! {
    SequelizeMeta (name) {
        name -> Varchar,
    }
}

diesel::table! {
    Users (id) {
        id -> Int4,
        firstName -> Nullable<Varchar>,
        lastName -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

diesel::table! {
    currencies (id) {
        id -> Int4,
        code -> Varchar,
        full_name -> Text,
        sign -> Varchar,
    }
}

diesel::table! {
    exchange_rates (id) {
        id -> Int4,
        base_currency_id -> Nullable<Int4>,
        target_currency_id -> Nullable<Int4>,
        rate -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    SequelizeMeta,
    Users,
    currencies,
    exchange_rates,
);
