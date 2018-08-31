#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate ws;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
use rocket_contrib::{Json};
use rocket::http::RawStr;
mod game;
use game::{Game};
use game::{Status};
mod user;
use user::{User};
use user::{UserName};
use user::{Score};
mod timeUtil;
use timeUtil::getCurrentTimeMilli;

mod wsServer;
use wsServer::{Server};
use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};
use std::rc::Rc;
use std::cell::Cell;
#[post("/", data = "<userName>")]
fn createUser(userName: Json<UserName>) -> Json<User> {
    Json(User{id:Some("userId".to_string()), name:userName.0.name, score:0, ts:getCurrentTimeMilli()})
}

#[put("/<userId>", data = "<score>")]
fn updateUser(userId: &RawStr, score: Json<Score>) -> Json<User> {
    Json(User{id:Some(userId.as_str().to_string()), name:"userName".to_string(), score:score.0.score, ts:getCurrentTimeMilli()})
}

#[post("/")]
fn createGame() -> Json<Game> {
    Json(
        Game{sessionId:Some("sessionId".to_string()), status:"create".to_string(), ts:getCurrentTimeMilli()}
    )
}

#[put("/<sessionId>", data = "<_status>")]
fn updateGame(sessionId: &RawStr, _status: Json<Status>) -> Json<Game> {
    Json(
        Game{sessionId:Some(sessionId.as_str().to_string()), status:_status.0.status, ts:getCurrentTimeMilli()}
    )
}

#[get("/<sessionId>")]
fn getGame(sessionId: &RawStr) -> Json<Game> {
    Json(
        Game{sessionId:Some(sessionId.as_str().to_string()), status:"test".to_string(), ts:getCurrentTimeMilli()}
    )
}

fn main() {
    // Cell gives us interior mutability so we can increment
    // or decrement the count between handlers.
    // Rc is a reference-counted box for sharing the count between handlers
    // since each handler needs to own its contents.
    let count = Rc::new(Cell::new(0));
    listen("0.0.0.0:3012", |out| { Server { out: out, count: count.clone() } }).unwrap();
    rocket::ignite()
        .mount("/rust-demo/v1/game/create-user", routes![createUser])
        .mount("/rust-demo/v1/game/user-status", routes![updateUser])
        .mount("/rust-demo/v1/game/status", routes![createGame, updateGame, getGame])
        .launch();
}
