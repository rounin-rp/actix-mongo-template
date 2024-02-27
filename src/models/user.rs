/*use crate::helpers::enums::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub user_status: UserStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub is_deleted: bool,
}
*/

use crate::{helpers::enums::UserStatus, traits::model::ModelTrait};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub user_status: UserStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub is_deleted: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCreateModel {
    pub first_name: String,
    pub last_name: String,
}

impl UserCreateModel {
    pub fn get_user_model(&self) -> UserModel {
        let mut user_model = UserModel::default();
        user_model.first_name = self.first_name.clone();
        user_model.last_name = self.last_name.clone();
        user_model
    }
}

impl ModelTrait for UserModel {
    fn set_created_at(&mut self, created_at: u64) {
        self.created_at = created_at;
    }
    fn set_id(&mut self, id: String) {
        self.id = id;
    }
    fn set_updated_at(&mut self, updated_at: u64) {
        self.updated_at = updated_at;
    }
}
