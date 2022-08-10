use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub uid: Option<i64>,
    pub name: String,
    pub login: String,
    pub hashword: String,
}
