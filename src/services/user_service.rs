use crate::{
    database::{core_service, mongodb::MongoClient},
    handlers::error_handler::Errors,
    models::user::*,
};
use mongodb::{
    bson::{self, doc},
    options::UpdateModifications,
};
pub async fn create_user(
    mongo_client: MongoClient,
    input: UserCreateModel,
) -> Result<UserModel, Errors> {
    let mut user_model = input.get_user_model();
    mongo_client
        .create_one("users", &mut user_model, None, None)
        .await?;
    Ok(user_model)
}

pub async fn get_all_users(mongo_client: MongoClient) -> Result<Vec<UserModel>, Errors> {
    mongo_client
        .read_many::<UserModel>("users", None, None)
        .await
}
