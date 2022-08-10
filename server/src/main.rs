#[macro_use]
extern crate rocket;
use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    response::status::{Created, NoContent, NotFound},
};

use rask_server::{
    ApiError,
    models::User,
    schema::users,
    Db,
};

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(Db::fairing())
        .mount(
            "/users",
            routes![
                register_user,
                list_users,
                retrieve_user,
                update_user,
                delete_user
            ],
        )
}

//TODO: diesel mysql does not support insert_into.get_results but postgres does. .execute will never error so we have redundant code. switch to postgresql database

//TODO: ensure login does not exist
//TODO: respond with something meaningful on success (not usize)
//TODO: replace .execute with .get_results after switching to postgres
#[post("/register", data = "<user>")]
async fn register_user(connection: Db, user: Json<User>
) -> Result<Created<Json<usize>>, Json<ApiError>> {
    connection
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(&user.into_inner())
                .execute(c)
        })
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}

#[get("/<uid>")]
async fn retrieve_user(connection: Db, uid: i64
) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| users::table.filter(users::uid.eq(uid)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[get("/list")]
async fn list_users(connection: Db
) -> Result<Json<Vec<User>>, Json<ApiError>> {
    connection
        .run(|c| users::table.load(c))
        .await
        .map(Json)
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}

//TODO: ensure login does not exist
//TODO: respond with something meaningful on success (not usize)
//TODO: replace .execute with .get_results after switching to postgres
#[patch("/update/<uid>", data = "<user>")]
async fn update_user(
    connection: Db,
    uid: i64,
    user: Json<User>,
) -> Result<Json<usize>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            diesel::update(users::table.find(uid))
                .set(&user.into_inner())
                .execute(c)
        })
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[delete("/delete/<uid>")]
async fn delete_user(connection: Db, uid: i64) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            let affected = diesel::delete(users::table.filter(users::uid.eq(uid)))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Err("NotFound"),
                _ => Err("Fundamental error"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
