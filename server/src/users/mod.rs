pub mod model;
// pub mod auth;

use bcrypt::{hash, verify};
use diesel::prelude::*;
use rocket::{
    post, get, patch, delete,
    serde::json::Json,
    response::status::{Created, NoContent, NotFound},
};
use crate::{ Db, ApiError, users::model::UnhashedUser, schema::users };
use self::model::User;

//TODO: ensure login does not exist
//TODO: respond with something meaningful on success (not usize)
//NOTE: using .get_results instead of .execute returns a user instead of 0 or 1
#[post("/register", data = "<unhashed_user>")]
pub async fn register(connection: Db, unhashed_user: Json<UnhashedUser>
) -> Result<Created<Json<usize>>, Json<ApiError>> {

    let hashword = match hash(unhashed_user.password.clone(), 4) {
        Ok(h) => h,
        Err(e) => {
            return Err(Json(ApiError {
                details: e.to_string(),
            }));
        }
    };

    let user = User {
        uid: None,
        name: unhashed_user.name.clone(),
        login: unhashed_user.login.clone(),
        hashword,
    };

    connection
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(&user)
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
pub async fn retrieve(connection: Db, uid: i32
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
pub async fn list(connection: Db
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
#[patch("/update/<uid>", data = "<user>")]
pub async fn update(
    connection: Db,
    uid: i32,
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
pub async fn delete(connection: Db, uid: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
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
