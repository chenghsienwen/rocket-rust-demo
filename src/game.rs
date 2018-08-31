#![recursion_limit="128"]
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::time::{SystemTime, UNIX_EPOCH};

mod schema {
    table! {
        games {
            id -> Nullable<Integer>,
            status -> Text,
            ts -> Integer,
        }
    }
}

use self::schema::games;
use self::schema::games::dsl::{games as all_games, status as game_status};

#[table_name="games"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
pub struct Game {
    pub id: Option<i32>,
    pub status: String,
    pub ts: i32
}
#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String
}
pub fn getCurrentTimeMilli() -> i32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos()  as u64 / 1_000_000) as i32
}
impl Game {
    pub fn all(conn: &SqliteConnection) -> Vec<Game> {
        all_games.order(games::ts.desc()).load::<Game>(conn).unwrap()
    }

    pub fn insert(id: i32, conn: &SqliteConnection) -> Game {
        let t = Game { id: Some(id), status: "create".to_string(), ts: getCurrentTimeMilli() };
        diesel::insert_into(games::table).values(&t).execute(conn).is_ok();
        t
    }

    pub fn get_with_id(id: i32, conn: &SqliteConnection) -> Game {
        all_games.find(id).get_result::<Game>(conn).unwrap()
    }

    pub fn update_with_id(id: i32, status: String, conn: &SqliteConnection) -> Game {
        let game = all_games.find(id).get_result::<Game>(conn);
        if game.is_err() {
            return Game { id: Some(id), status: "not found".to_string(), ts: getCurrentTimeMilli() };
        }

        let new_status = status;
        let updated_game = diesel::update(all_games.find(id));
        updated_game.set(game_status.eq(new_status)).execute(conn).is_ok();
        all_games.find(id).get_result::<Game>(conn).unwrap()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_games.find(id)).execute(conn).is_ok()
    }
}
