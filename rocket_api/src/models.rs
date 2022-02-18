use diesel::{PgConnection, QueryDsl};

use diesel::prelude::*;

#[derive(Queryable,Serialize,Clone)]
pub struct Parent {
    id: i32,
    first_name: String,
    last_name: String,
    full_name: String,
    contact: String,
    city: String,
    job_status: String,
    age: String,
    gender: String,
    job_type: String,
    email: String,
    photo: String,
    thumbnail: String,
}


impl Parent {
    pub fn get(parent_id: i32, conn: &PgConnection) -> Vec<Parent> {
        use super::schema::parents::dsl::*;
        parents
            .find(parent_id)
            .load::<Parent>(conn)
            .expect(&format!("Error loading book with id:{:?}", id))
    }

    pub fn get_all(conn: &PgConnection) -> Vec<Parent> {
        use super::schema::parents::dsl::*;
        parents
            .order(first_name.desc())
            .load::<Parent>(conn)
            .expect("Error loading parents")
    }

    pub fn update_by_id(parent_id: i32, parent: NewParent, conn: &PgConnection) -> bool {
        use super::schema::parents::dsl::*;

        diesel::update(parents
            .find(parent_id)
        )
            .set(&parent)
            .execute(conn).is_ok()
    }
    pub fn insert(parent: NewParent, conn: &PgConnection) -> bool {
        use super::schema::parents::dsl::*;
        diesel::insert_into(parents)
            .values(&parent)
            .execute(conn)
            .is_ok()
    }
    pub fn delete_by_id(parent_id: i32, conn: &PgConnection) -> bool {
        use super::schema::parents::dsl::*;
        diesel::delete(parents.find(parent_id))
            .execute(conn)
            .is_ok()
    }
    pub fn delete_all(conn: &PgConnection) -> bool {
        use super::schema::parents::dsl::*;
        diesel::delete(parents)
            .execute(conn)
            .is_ok()
    }
}

use super::schema::parents;

#[derive(Insertable, AsChangeset,Serialize,Deserialize)]
#[table_name = "parents"]
pub struct NewParent {
    first_name: String,
    last_name: String,
    full_name: String,
    contact: String,
    city: String,
    job_status: String,
    age: String,
    gender: String,
    job_type: String,
    email: String,
    photo: String,
    thumbnail: String,
}

impl NewParent {
   pub fn new(
        first_name: String,
        last_name: String,
        full_name: String,
        contact: String,
        city: String,
        job_status: String,
        age: String,
        gender: String,
        job_type: String,
        email: String,
        photo: String,
        thumbnail: String,
    ) -> NewParent {
        NewParent {
            first_name,
            last_name,
            full_name,
            contact,
            city,
            job_status,
            age,
            gender,
            job_type,
            email,
            photo,
            thumbnail,
        }
    }
}