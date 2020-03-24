table! {
    authors (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    reference_authors (id) {
        id -> Int4,
        reference_id -> Int4,
        author_id -> Int4,
    }
}

table! {
    reference_item_submissions (id) {
        id -> Int4,
        reference_item_id -> Int4,
        submitting_user_id -> Int4,
        is_public -> Bool,
    }
}

table! {
    reference_items (id) {
        id -> Int4,
        title -> Text,
        url -> Nullable<Text>,
    }
}

table! {
    sessions (token) {
        token -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
        last_accessed_at -> Timestamptz,
        accessed_by_client_ip -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        passhash -> Varchar,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    reference_authors,
    reference_item_submissions,
    reference_items,
    sessions,
    users,
);
