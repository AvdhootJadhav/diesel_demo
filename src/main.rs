use diesel::{RunQueryDsl, r2d2::{ConnectionManager, Pool}, PgConnection, ExpressionMethods};
use diesel_demo::{
    db::establish_connection,
    models::{Comics, Response},
    schema::comics,
};

use rocket::{serde::json::Json, State};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

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
    use crate::comics::dsl::*;
    let data = comics.load::<Comics>(&mut pool.get().unwrap()).unwrap();

    Json::from(data)
}

#[patch("/update", data = "<data>")]
fn edit_comic(pool: &State<Pool<ConnectionManager<PgConnection>>>, data: Json<Comics>) -> Json<Response> {
    let Json(data) = data;

    let res = diesel::update(comics::table).filter(comics::id.eq(data.id))
        .set((comics::title.eq(data.title), comics::status.eq(data.status), comics::author.eq(data.author)))
        .execute(&mut pool.get().unwrap()).unwrap();

    let response = Response {
        status: res > 0
    };

    Json::from(response)

}

#[delete("/comic/<id>")]
fn delete_comic(pool: &State<Pool<ConnectionManager<PgConnection>>>, id: i32) -> Json<Response> {

    let res = diesel::delete(comics::table)
        .filter(comics::id.eq(id))
        .execute(&mut pool.get().unwrap()).unwrap();

    let response = Response{
        status: res > 0
    };

    Json::from(response)
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
        .mount("/", routes![save_comic, get_all_comics, edit_comic, delete_comic])
}
