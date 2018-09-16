table! {
    games (id) {
        id -> Int4,
        status -> Varchar,
        ts -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        score -> Int4,
        ts -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    games,
    users,
);
