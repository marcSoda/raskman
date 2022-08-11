use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub uid: Option<i32>,
    pub name: String,
    pub login: String,
    pub hashword: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnhashedUser {
    pub name: String,
    pub login: String,
    pub password: String,
}
