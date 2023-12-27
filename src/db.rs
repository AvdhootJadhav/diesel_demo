use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(url);
    
    Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Failed to create pool")
}
