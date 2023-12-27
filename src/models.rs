use diesel::{deserialize::Queryable, Selectable, prelude::Insertable};
use rocket::serde::{Deserialize, self};

#[derive(Queryable, Selectable, Deserialize, Debug, serde::Serialize, Insertable)]
#[diesel(table_name = crate::schema::comics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Comics {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub status: String
}

#[derive(serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub status: bool
}