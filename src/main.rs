use diesel::{RunQueryDsl, r2d2::{ConnectionManager, Pool}, PgConnection};
use diesel_demo::{
    db::establish_connection,
    models::Comics,
    schema::comics,
};

use rocket::{serde::json::Json, State};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use crate::comics::dsl::*;

#[macro_use]
extern crate rocket;

#[post("/new", data = "<data>")]
fn save_comic(pool: &State<Pool<ConnectionManager<PgConnection>>>, data: Json<Comics>) -> Json<Comics> {

    let Json(data) = data;

    diesel::insert_into(comics::table)
        .values(&data)
        .execute(&mut pool.get().unwrap())
        .expect("Error saving comic");

    return Json::from(data);
}

#[get("/all")]
fn get_all_comics(pool: &State<Pool<ConnectionManager<PgConnection>>>) -> Json<Vec<Comics>> {
    let data = comics.load::<Comics>(&mut pool.get().unwrap()).unwrap();

    Json::from(data)
}

#[launch]
fn rocket() -> _ {
    let pool = establish_connection();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subcriber failed");
    rocket::build()
        .manage(pool)
        .mount("/", routes![save_comic, get_all_comics])
}
