#![recursion_limit="128"]
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
// mod timeUtil;
// use timeUtil::getCurrentTimeMilli;
use std::time::{SystemTime, UNIX_EPOCH};

mod schema {
    table! {
        users {
            id -> Nullable<Integer>,
            name -> Text,
            score -> Integer,
            ts -> Integer,
        }
    }
}

use self::schema::users;
use self::schema::users::dsl::{users as all_users, score as user_score};

#[table_name="users"]
#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub score: i32,
    pub ts: i32
}
#[derive(Serialize, Deserialize)]
pub struct UserName {
    pub name: String
}
#[derive(Serialize, Deserialize)]
pub struct Score {
    pub score: i32
}

pub fn getCurrentTimeMilli() -> i32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos()  as u64 / 1_000_000) as i32
}

impl User {
    pub fn all(conn: &SqliteConnection) -> Vec<User> {
        all_users.order(users::score.desc()).load::<User>(conn).unwrap()
    }

    pub fn insert(name: UserName, conn: &SqliteConnection) -> User {
        let t = User { id: None, name: name.name, score: 0, ts: getCurrentTimeMilli() };
        diesel::insert_into(users::table).values(&t).execute(conn).is_ok();
        t
    }

    pub fn get_with_id(id: i32, conn: &SqliteConnection) -> User {
        all_users.find(id).get_result::<User>(conn).unwrap()
    }

    pub fn update_with_id(id: i32, score: i32, conn: &SqliteConnection) -> User {
        let user = all_users.find(id).get_result::<User>(conn);
        if user.is_err() {
            return User { id: Some(id), name:"null".to_string(), score: 0, ts: getCurrentTimeMilli() };
        }

        let new_score = score;
        let updated_user = diesel::update(all_users.find(id));
        updated_user.set(user_score.eq(new_score)).execute(conn).is_ok();
        all_users.find(id).get_result::<User>(conn).unwrap()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_users.find(id)).execute(conn).is_ok()
    }
}
