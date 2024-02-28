use crate::handlers::error_handler::Errors;
use crate::models::user::UserModel;
use crate::traits::jwt::JwtToken;
use crate::{
    database::mongodb::MongoClient, models::user::UserCreateModel, services::user_service,
};
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use serde::de::DeserializeOwned;
use serde::Serialize;

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

#[get("/all")]
pub async fn get_all_users(
    auth_token: JwtToken,
    mongo_client: web::Data<MongoClient>,
) -> impl Responder {
    let response = user_service::get_all_users(mongo_client.get_ref().clone()).await;
    handle_json_response::<Vec<UserModel>>(response)
}

fn handle_json_response<Model: Serialize + DeserializeOwned + Clone>(
    response: Result<Model, Errors>,
) -> impl Responder {
    match response {
        Ok(message) => HttpResponse::Created().json(message),
        Err(error) => error.error_response(),
    }
}

pub fn routes() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("users")
        .service(create_user)
        .service(get_all_users)
        .service(health_check)
}
