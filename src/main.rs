// https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
#[macro_use]
extern crate diesel;

mod controller;
mod schema;
mod db;
mod error_handler;
mod readings;

use std::env;
use actix_web::{App, HttpServer, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use listenfd::ListenFd;
use crate::schema::measurements_single_location_function::measurements;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(readings::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please configure host in .env");
            let port = env::var("PORT").expect("Please configure port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await

    /*
    HttpServer::new(move || {
        App::new()
            .data(web::PayloadConfig::new(1 << 25))
            .data(pool.clone())
            .route("/", web::get().to(controller::get_root))
            .route("/locations", web::get().to(controller::get_locations))
            .route(
                "/locations/{id}",
                web::get().to(controller::get_location_by_id),
            )
            .route("/readings", web::get().to(controller::get_readings))
            .route(
                "/readings/{id}",
                web::get().to(controller::get_readings_by_id),
            )
            .route(
                "/location_readings",
                web::get().to(controller::get_location_readings),
            )
            .route(
                "/measurements_single_location/{id}/{rows}",
                web::get().to(controller::get_measurements_single_location),
            )
    })
    .bind("0.0.0.0:8090")?
    .run()
    .await*/
}
