use actix_cors::Cors;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

pub mod database;
pub mod handlers;
pub mod traits;

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
