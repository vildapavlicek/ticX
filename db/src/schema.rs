table! {
    tickets (id) {
        id -> Int4,
        author_id -> Int4,
        description -> Varchar,
        severity -> Int2,
        status -> Int2,
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

allow_tables_to_appear_in_same_query!(tickets, users,);
