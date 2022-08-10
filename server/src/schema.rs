table! {
    //NOTE: ensure that uid is nullable. After diesel migration run, the Nullable wrapper gets removed
    users (uid) {
        uid -> Nullable<Int4>,
        name -> Varchar,
        login -> Varchar,
        hashword -> Varchar,
    }
}
