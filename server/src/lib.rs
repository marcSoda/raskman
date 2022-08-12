pub mod schema;
pub mod users;
pub mod task_lists;

#[macro_use]
extern crate diesel;

use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::database;

#[database("my_db")]
pub struct Db(diesel::PgConnection);


#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}
