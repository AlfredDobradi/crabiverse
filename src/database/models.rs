use diesel::prelude::*;
use serde::{ Serialize};
use crate::database::schema::posts;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    // pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}