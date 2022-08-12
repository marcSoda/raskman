use diesel::{ Insertable, Queryable };
use serde::{ Deserialize, Serialize };
use crate::{ schema::task_lists, users::models::Credentials };

#[derive(Serialize, Deserialize, Insertable, Clone, Queryable, QueryableByName, Debug, AsChangeset)]
#[table_name = "task_lists"]
pub struct TaskList {
    pub tid: Option<i32>,
    pub uid: i32,
    pub task_list: Option<serde_json::Value>,
    // pub task_list: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UploadPacket {
    pub credentials: Credentials,
    pub task_list: Option<serde_json::Value>,
    // pub task_list: Vec<u8>,
}
