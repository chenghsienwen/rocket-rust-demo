use diesel;
use diesel::prelude::*;
use std::time::SystemTime;
mod schema {
    table! {
        games {
            id -> Integer,
            status -> Text,
            ts -> Timestamp,
        }
    }
}

use self::schema::games;
use self::schema::games::dsl::{games as all_games, status as game_status};

#[table_name="games"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
pub struct Game {
    pub id: i32,
    pub status: String,
    pub ts: SystemTime
}
#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String
}

impl Game {
    pub fn all(conn: &PgConnection) -> Vec<Game> {
        all_games.order(games::ts.desc()).load::<Game>(conn).unwrap()
    }

    pub fn insert(_id: i32, conn: &PgConnection) -> Game {
        let t = Game { id: _id, status: "create".to_string(), ts: SystemTime::now() };
        diesel::insert_into(games::table).values(&t).execute(conn).is_ok();
        t
    }

    pub fn get_with_id(_id: i32, conn: &PgConnection) -> Game {
        all_games.find(_id).get_result::<Game>(conn).unwrap()
    }

    pub fn update_with_id(_id: i32, status: String, conn: &PgConnection) -> Game {
        let game = all_games.find(_id).get_result::<Game>(conn);
        if game.is_err() {
            return Game { id: _id, status: "not found".to_string(), ts: SystemTime::now() };
        }

        let new_status = status;
        let updated_game = diesel::update(all_games.find(_id));
        updated_game.set(game_status.eq(new_status)).execute(conn).is_ok();
        all_games.find(_id).get_result::<Game>(conn).unwrap()
    }

    pub fn delete_with_id(_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(all_games.find(_id)).execute(conn).is_ok()
    }
}
