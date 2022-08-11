// use rocket;
use rocket::{ launch, routes, Rocket, Build };
use rask_server::{Db, users};

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
                users::delete
            ],
        )
}
