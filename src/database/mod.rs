use diesel::pg::PgConnection;
use diesel::prelude::*;

use self::models::Post;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://crabiverse@127.0.0.1:5432/crabiverse_dev?sslmode=disable";
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_posts() -> Result<Vec<Post>, diesel::result::Error> {
    use self::schema::posts::dsl::*;
    use models::*;

    let connection = &mut establish_connection();
    posts.load::<Post>(connection)
}

pub fn create_post(title: &str, body: &str) -> Result<(), diesel::result::Error> {
    use crate::database::schema::posts;
    use self::models::{NewPost};

    let conn = &mut establish_connection();

    let new_post = NewPost{ title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(conn)?;
    Ok(())
}