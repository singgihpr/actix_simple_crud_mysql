use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;

mod db;
mod handlers;
mod models;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = db::establish_connection().await;

    info!("Starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(handlers::create_user)
            .service(handlers::get_users)
            .service(handlers::get_user_by_id)
            .service(handlers::update_user)
            .service(handlers::delete_user)
            .service(handlers::insert_10k_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}