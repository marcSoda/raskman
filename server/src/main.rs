use rocket::{ launch, routes, Rocket, Build };
use rask_server::{Db, users, task_lists};

#[launch]
pub fn rocket() -> Rocket<Build> {
    let rocket = rocket::build();
    rocket
        .attach(Db::fairing())
        .mount(
            "/users",
            routes![
                users::register,
                users::list,
                users::retrieve,
                users::update,
                users::delete,
                users::login,
                task_lists::upload,
                task_lists::retrieve,
            ],
        )
}
