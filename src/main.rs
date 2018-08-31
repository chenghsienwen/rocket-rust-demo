#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
// extern crate ws;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
use rocket::Rocket;
use rocket_contrib::{Json, Value};
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

// mod wsServer;
// use wsServer::{Server};
// use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};
// use std::rc::Rc;
// use std::cell::Cell;

mod static_files;
mod db;
use rocket_contrib::Template;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
mod task;
use task::{Task};

#[derive(Debug, Serialize)]
struct Context<'a, 'b>{ msg: Option<(&'a str, &'b str)>, tasks: Vec<Task> }

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &db::Conn, msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), tasks: Task::all(conn)}
    }

    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, tasks: Task::all(conn)}
    }
}

#[get("/<id>")]
fn getUserList(id: i32, conn: db::Conn) -> Json<Value> {
    Json(json!(User::all(&conn)))
}

#[post("/<userId>", data = "<userName>")]
fn createUser(userId: i32, userName: Json<UserName>, conn: db::Conn) -> Json<User> {
    //Json(User{id:Some(1), name:userName.0.name, score:0, ts:getCurrentTimeMilli()});
    Json(User::insert(userId, userName.0, &conn))
}

#[put("/<userId>", data = "<score>")]
fn updateUser(userId: i32, score: Json<Score>, conn: db::Conn) -> Json<User> {
    //Json(User{id:Some(userId), name:"userName".to_string(), score:score.0.score, ts:getCurrentTimeMilli()})
    Json(User::update_with_id(userId, score.0.score, &conn))
}

#[post("/<id>")]
fn createGame(id: i32, conn: db::Conn) -> Json<Game> {
    // Json(
    //     Game{id:Some(1), status:"create".to_string(), ts:getCurrentTimeMilli()}
    // )
    Json(Game::insert(id, &conn))
}

#[put("/<id>", data = "<_status>")]
fn updateGame(id: i32, _status: Json<Status>, conn: db::Conn) -> Json<Game> {
    // Json(
    //     Game{id:Some(id), status:_status.0.status, ts:getCurrentTimeMilli()}
    // )
    Json(Game::update_with_id(id, _status.0.status, &conn))
}

#[get("/<id>")]
fn getGame(id: i32, conn: db::Conn) -> Json<Game> {
    // Json(
    //     Game{id:Some(id), status:"test".to_string(), ts:getCurrentTimeMilli()}
    // )
    Json(Game::get_with_id(id, &conn))
}

#[get("/")]
fn index(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("index", &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
    })
}

fn rocket() -> (Rocket, Option<db::Conn>) {
    let pool = db::init_pool();
    let conn = if cfg!(test) {
        Some(db::Conn(pool.get().expect("database connection for testing")))
    } else {
        None
    };

    let rocket = rocket::ignite()
        .manage(pool)
        .mount("/", routes![index, static_files::all])
        .mount("/rust-demo/v1/game/create-user", routes![createUser])
        .mount("/rust-demo/v1/game/user-status", routes![updateUser])
        .mount("/rust-demo/v1/game/users", routes![getUserList])
        .mount("/rust-demo/v1/game/status", routes![createGame, updateGame, getGame])
        .attach(Template::fairing());

    (rocket, conn)
}

fn main() {
    // Cell gives us interior mutability so we can increment
    // or decrement the count between handlers.
    // Rc is a reference-counted box for sharing the count between handlers
    // since each handler needs to own its contents.
    // let count = Rc::new(Cell::new(0));
    // listen("0.0.0.0:3012", |out| { Server { out: out, count: count.clone() } }).unwrap();

    rocket().0.launch();
}
