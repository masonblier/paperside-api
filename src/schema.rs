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
    reference_items (id) {
        id -> Int4,
        title -> Text,
        url -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    reference_authors,
    reference_items,
);
