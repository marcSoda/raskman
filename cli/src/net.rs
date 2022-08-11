extern crate reqwest;

use std::error::Error;
use serde::{ Serialize, Deserialize };
use crate::app::errors::{ LoginError, RegisterError };
use reqwest::blocking::Client;
use std::fs::OpenOptions;
use std::io::{
    SeekFrom,
    prelude::Seek
};

//TODO: make register error more verbose after configuring the server to return a better response

#[derive(Serialize, Deserialize, Debug)]
struct User {
    uid: Option<i64>,
    name: String,
    login: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
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
        _ => return Err(Box::new(RegisterError("Failed to register: an unknown error occurred"))),
    };

    Ok(())
}

//todo: more meaningful response on error (internal server error vs failed to login)
pub fn login(login: String, password: String) -> Result<(), Box<dyn Error>> {
    let creds = Credentials {
        login,
        password,
    };

    let request_url = "http://147.182.135.192:8000/users/login";
    let response = Client::new()
        .get(request_url)
        .json(&creds)
        .send()?;

    match response.json()? {
        1 => debug!("Successful registration"),
        _ => return Err(Box::new(LoginError("Failed to login"))),
    };

    //open file for writing. creat if not exists
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("creds.json")?;
    //read file into rask struct
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer_pretty(&file, &creds)?;

    Ok(())
}
