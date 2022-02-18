table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}

table! {
    parents (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        full_name -> Varchar,
        contact -> Varchar,
        city -> Varchar,
        job_status -> Varchar,
        age -> Varchar,
        gender -> Varchar,
        job_type -> Varchar,
        email -> Varchar,
        photo -> Varchar,
        thumbnail -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    parents,
);
