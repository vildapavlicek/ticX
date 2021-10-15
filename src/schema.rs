table! {
    tickets (id) {
        id -> Int4,
        author_id -> Nullable<Int4>,
        description -> Varchar,
        severity -> Nullable<Int2>,
        status -> Nullable<Int2>,
        created -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        created -> Timestamptz,
    }
}

joinable!(tickets -> users (author_id));

allow_tables_to_appear_in_same_query!(
    tickets,
    users,
);
