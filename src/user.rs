
use diesel;
use diesel::prelude::*;
use serde_derive;

use std::time::SystemTime;
mod schema {
    table! {
        users {
            id -> Integer,
            game_id -> Integer,
            name -> Text,
            score -> Integer,
            ts -> Timestamp,
        }
    }
}

use self::schema::users;
use self::schema::users::columns::*;
use self::schema::users::dsl::{users as all_users, score as user_score};

#[derive(Serialize, Deserialize, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub score: i32,
    pub ts: SystemTime
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserForm<'a> {
    game_id: i32,
    name: &'a str
}

#[derive(Serialize, Deserialize)]
pub struct UserName {
    pub name: String
}
#[derive(Serialize, Deserialize)]
pub struct Score {
    pub score: i32
}

trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}


impl User {
    pub fn insert_default_values(conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(all_users).default_values().execute(conn)
    }

    pub fn all(gameId: i32, conn: &PgConnection) -> Vec<User> {
        all_users.filter(game_id.eq(gameId)).order(users::score.desc()).load::<User>(conn).unwrap()
    }

    pub fn insert(gameId: i32, userName: UserName, conn: &PgConnection) -> Vec<User> {
        let request = UserForm {game_id: gameId, name: &userName.name};
        diesel::insert_into(users::table).values(&request).get_results::<User>(conn).unwrap()
    }

    pub fn get_with_id(_id: i32, conn: &PgConnection) -> User {
        all_users.find(_id).get_result::<User>(conn).unwrap()
    }

    pub fn update_with_id(_id: i32, new_score: i32, conn: &PgConnection) -> User {
        let user = all_users.find(_id).get_result::<User>(conn);
        if user.is_err() {
            return User { id: _id, game_id: 0, name:"null".to_string(), score: 0, ts: SystemTime::now() };
        }
        
        let updated_user = diesel::update(all_users.find(_id));
        updated_user.set(user_score.eq(new_score)).execute(conn).is_ok();
        all_users.find(_id).get_result::<User>(conn).unwrap()
    }

    pub fn delete_with_id(_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(all_users.find(_id)).execute(conn).is_ok()
    }
}
