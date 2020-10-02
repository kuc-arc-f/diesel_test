#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;

use diesel::prelude::*;
use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use diesel::sql_query;

mod lib;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
use schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
//
fn main() {
    println!("#start-db");
    use self::schema::posts::dsl::{id, posts};

    let connection = lib::establish_connection();

    let new_post = NewPost {
        title: "t1".to_string(),
        body: "b1".to_string(),
    };
    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&connection)
        .expect("Error saving new post");    
}
