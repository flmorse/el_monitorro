table! {
    feed_items (id) {
        id -> Int4,
        feed_id -> Int4,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        link -> Nullable<Text>,
        author -> Nullable<Text>,
        guid -> Nullable<Text>,
        publication_date -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    feeds (id) {
        id -> Int4,
        title -> Nullable<Text>,
        link -> Text,
        error -> Nullable<Text>,
        description -> Nullable<Text>,
        synced_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    tasks (id) {
        id -> Int8,
        job_uuid -> Text,
        status -> Text,
        result -> Nullable<Text>,
        run_at -> Nullable<Int8>,
        queue -> Nullable<Text>,
        attempts -> Int4,
        max_attempts -> Int4,
        created_at -> Int8,
        updated_at -> Int8,
        cron -> Nullable<Text>,
        interval -> Nullable<Int8>,
        job -> Text,
    }
}

joinable!(feed_items -> feeds (feed_id));

allow_tables_to_appear_in_same_query!(
    feed_items,
    feeds,
    tasks,
);