pub mod models;

use bcrypt::verify;
use diesel::{ prelude::*, sql_query };
use rocket::{
    post, get,
    serde::json::Json,
    response::status::{Created, NoContent, NotFound},
};
use crate::{
    Db, ApiError,
    schema::task_lists,
    users::{
        get_user_by_login,
        Credentials,
    }
};
use self::models::{ TaskList, UploadPacket };

//TODO: move auth to separate function. code duplication is ugly
#[post("/upload", data = "<pack>")]
pub async fn upload(connection: Db, pack: Json<UploadPacket>
) -> Result<Created<Json<usize>>, Json<ApiError>> {

    let user = match get_user_by_login(&connection, pack.credentials.login.clone()).await {
        Ok(u) => u,
        Err(_) => {
            return Err(Json(ApiError {
                details: "Error: login not found".to_string(),
            }));
        }
    };

    let user_clone = user.clone(); //quick and dirty solution the following closure capturing user

    match verify(pack.credentials.password.clone(), &user.hashword.clone()) {
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

    connection
        .run(move |c| {
            let affected = diesel::delete(
                task_lists::table.filter(task_lists::uid.eq(user_clone.uid.unwrap())))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Ok(()),
                _ => Err("Fundamental error"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        });

    let tl = TaskList {
        tid: None,
        uid: user.uid.unwrap(),
        task_list: Some(serde_json::to_value(pack.task_list.clone()).unwrap()),
    };

    connection
        .run(move |c| {
            diesel::insert_into(task_lists::table)
                .values(&tl)
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

#[get("/retrieve", data = "<creds>")]
pub async fn retrieve(connection: Db, creds: Json<Credentials>
) -> Result<Json<TaskList>, Json<ApiError>> {

    let user = match get_user_by_login(&connection, creds.login.clone()).await {
        Ok(u) => u,
        Err(_) => {
            return Err(Json(ApiError {
                details: "Error: login not found".to_string(),
            }));
        }
    };

    let user_clone = user.clone(); //quick and dirty solution the following closure capturing user

    match verify(creds.password.clone(), &user.hashword.clone()) {
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

    connection
        .run(move |c| task_lists::table.filter(task_lists::uid.eq(user.uid.unwrap())).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}
