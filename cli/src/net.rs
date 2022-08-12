extern crate reqwest;

use std::error::Error;
use serde::{ Serialize, Deserialize };
use crate::{
    app::task::Task,
    app::errors::{ LoginError, RegisterError }
};
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
pub struct Credentials {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadPacket {
    pub credentials: Credentials,
    pub task_list: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tid: Option<i32>,
    pub uid: i32,
    pub task_list: Option<serde_json::Value>,
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

pub fn push(task_list: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("creds.json")?;
    //read file into rask struct
    let creds: Credentials = match serde_json::from_reader(&file) {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };

    let stl = serde_json::to_value(&task_list)?;

    let pack = UploadPacket {
        credentials: creds,
        task_list: Some(stl),
    };

    let request_url = "http://147.182.135.192:8000/users/upload";
    let response = Client::new()
        .post(request_url)
        .json(&pack)
        .send()?;

    match response.json()? {
        1 => debug!("Successful push"),
        _ => return Err(Box::new(LoginError("Failed to push"))),
    };

    Ok(())
}

//TODO: I very lazily return the whole TaskList struct from the server (which contains tid and uid). Refactor server to only return task_list so the cli need not be concerned with this information
pub fn pull() -> Result<Vec<Task>, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("creds.json")?;
    //read file into rask struct
    let creds: Credentials = match serde_json::from_reader(&file) {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };

    let request_url = "http://147.182.135.192:8000/users/retrieve";
    let response = Client::new()
        .get(request_url)
        .json(&creds)
        .send()?;

    let tlr: TaskList = serde_json::from_value(response.json()?)?;
    let tl: Vec<Task> = serde_json::from_value(tlr.task_list.unwrap())?;
    Ok(tl)
}
