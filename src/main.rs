#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
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
    rocket::ignite()
        .mount("/rust-demo/v1/game/create-user", routes![createUser])
        .mount("/rust-demo/v1/game/user-status", routes![updateUser])
        .mount("/rust-demo/v1/game/status", routes![createGame, updateGame, getGame])
        .launch();
}
