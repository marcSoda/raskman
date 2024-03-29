table! {
    task_lists (tid) {
        tid -> Nullable<Int4>,
        uid -> Int4,
        task_list -> Nullable<Jsonb>,
    }
}

table! {
    users (uid) {
        uid -> Nullable<Int4>,
        name -> Varchar,
        login -> Varchar,
        hashword -> Varchar,
    }
}

joinable!(task_lists -> users (uid));

allow_tables_to_appear_in_same_query!(
    task_lists,
    users,
);
