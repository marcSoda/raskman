table! {
    task_lists (tid) {
        tid -> Int4,
        uid -> Int4,
        task_list -> Bytea,
    }
}

table! {
    users (uid) {
        uid -> Int4,
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
