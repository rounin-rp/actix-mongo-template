use crate::{
    database::mongodb::MongoClient, models::user::UserCreateModel, services::user_service,
};
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Users Ok")
}

#[post("/create")]
pub async fn create_user(
    mongo_client: web::Data<MongoClient>,
    input: web::Json<UserCreateModel>,
) -> impl Responder {
    let response = user_service::create_user(mongo_client.get_ref().clone(), input.clone()).await;
    match response {
        Ok(message) => HttpResponse::Created().json(message),
        Err(error) => error.error_response(),
    }
}

pub fn routes() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("users")
        .service(create_user)
        .service(health_check)
}
