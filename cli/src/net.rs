extern crate reqwest;

use std::error::Error;
use serde::{ Serialize, Deserialize };
use crate::app::errors::RegisterError;
use reqwest::blocking::Client;

//TODO: make register error more verbose after configuring the server to return a better response

#[derive(Serialize, Deserialize, Debug)]
struct User {
    uid: Option<i64>,
    name: String,
    login: String,
    password: String,
}

pub fn register(name: String, login: String, password: String) -> Result<(), Box<dyn Error>> {
    let user = User {
        uid: None,
        name,
        login,
        password,
    };

    let request_url = "http://147.182.135.192:8000/users/register";
    let response = Client::new()
        .post(request_url)
        .json(&user)
        .send()?;

    match response.json()? {
        1 => debug!("Successful registration"),
        0 => return Err(Box::new(RegisterError("Failed to register"))),
        _ => return Err(Box::new(RegisterError("Failed to reguster: an unknown error occurred"))),
    };

    Ok(())
}
