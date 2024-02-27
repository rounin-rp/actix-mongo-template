use actix_web::web;

pub mod user_routes;

pub fn routes() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("api").service(user_routes::routes())
}
