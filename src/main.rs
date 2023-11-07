mod handlers;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<PgConnection>::new("postgres://username:password@localhost/mydatabase");
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::user::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
