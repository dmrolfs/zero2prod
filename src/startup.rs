use std::sync::{Arc, Mutex};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use sonyflake::Sonyflake;
use tracing_actix_web::TracingLogger;

use crate::routes::health_check;
use crate::routes::subscribe;

lazy_static::lazy_static! {
    static ref SONYFLAKE: Arc<Mutex<Sonyflake>> = {
        Arc::new(Mutex::new(Sonyflake::new().unwrap()))
    };
}

pub fn next_id() -> u64 {
    let mut sf = SONYFLAKE.lock().unwrap();
    sf.next_id().unwrap()
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
