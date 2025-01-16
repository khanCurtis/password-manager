use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{QueryResult, MysqlPool, prelude::*};

#[derive(Database)]
#[database("diesel_mysql")]
struct Db(MysqlPool);

#[derive(Queryable, Insertable)]
#[diesel(table_name = posts)]
struct Post {
    id: i64,
    title: String,
    published: bool,
}

diesel::table! {
    posts (id) {
        id -> BigInt,
        title -> Text,
        published -> Bool,
    }
}

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

#[get("/")]
async fn list(mut db: Connection<Db>) -> QueryResult<String> {
    let post_ids: Vec<i64> = posts::table
        .select(posts::id)
        .load(&mut db)
        .await?;

    Ok(format!("{post_ids:?}"))
}

fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
