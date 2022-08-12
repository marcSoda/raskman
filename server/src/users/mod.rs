pub mod models;

use bcrypt::{hash, verify};
use diesel::prelude::*;
use rocket::{
    post, get, patch, delete,
    serde::json::Json,
    response::status::{Created, NoContent, NotFound},
};
use crate::{ Db, ApiError, schema::users };
pub use self::models::{
    User, UnhashedUser, Credentials
};

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

#[get("/login", data = "<credentials>")]
pub async fn login(connection: Db, credentials: Json<Credentials>
) -> Result<Json<usize>, Json<ApiError>> {
    let user = match get_user_by_login(&connection, credentials.login.clone()).await {
        Ok(u) => u,
        Err(_) => {
            return Err(Json(ApiError {
                details: "Error: login not found".to_string(),
            }));
        }
    };

    match verify(credentials.password.clone(), &user.hashword.clone()) {
        Ok(res) => {
            match res {
                true => true,
                false => {
                    return Err(Json(ApiError {
                        details: "Error: wrong password".to_string(),
                    }));
                }
            }
        }, Err(_) => {
            return Err(Json(ApiError {
                details: "Error: internal server error".to_string(),
            }));
        }
    };

    Ok(Json(1))
}

#[get("/<uid>")]
pub async fn retrieve(connection: Db, uid: i32
) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    get_user_by_uid(connection, uid).await
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

pub async fn get_user_by_login(connection: &Db, login: String
) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| users::table.filter(users::login.eq(login)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

pub async fn get_user_by_uid(connection:Db, uid: i32
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
