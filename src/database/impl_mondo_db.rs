use bcrypt::verify;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Database};
use rocket::serde::json::Json;

use crate::database::connect_to_db::MongoDB;
use crate::database::{LoginError, RegistrationError};
use crate::helper::hash_text;
use crate::models::model_user::User;
use crate::private::{JWT_SECRET, REFRESH_JWT_SECRET};
use crate::routes::authorization::token::create_token::{
    create_token_and_refresh,
};
use crate::routes::routes_models::login_request::LoginRequest;
use crate::routes::routes_models::registration_request::RegistrationRequest;

impl MongoDB {
    pub fn new(database: Database) -> Self {
        MongoDB { database }
    }

    pub async fn find_user_by_login(&self, login: String) -> mongodb::error::Result<Option<User>> {
        let collection_user = self.database.collection::<User>("user");

        Ok(collection_user
            .find_one(bson::doc! { "login": login }, None)
            .await?)
    }

    pub async fn login(
        &self,
        login_request: Json<LoginRequest>,
    ) -> mongodb::error::Result<LoginError> {
        match Self::find_user_by_login(self, login_request.login.clone()).await {
            Ok(option_user) => match option_user {
                None => Ok(LoginError::WrongLogin),
                Some(user) => match verify(&login_request.password, &user.password) {
                    Ok(true) => Ok(LoginError::Ok),
                    Ok(false) => Ok(LoginError::WrongPassword),
                    Err(_) => Ok(LoginError::WrongPassword),
                },
            },
            Err(_) => Ok(LoginError::WrongLogin),
        }
    }

    pub async fn registration(
        &self,
        registration_request: Json<RegistrationRequest>,
    ) -> mongodb::error::Result<RegistrationError> {
        let collection_user = self.database.collection::<User>("user");
        match Self::find_user_by_login(self, registration_request.login.clone()).await {
            Ok(Some(_)) => Ok(RegistrationError::AlreadyRegistered),
            Ok(None) => match hash_text(registration_request.password.clone(), 4) {
                Ok(hash_password) => {
                    let user = User {
                        _id: ObjectId::new(),
                        login: registration_request.login.clone(),
                        password: hash_password,
                        first_name: registration_request.first_name.clone(),
                        last_name: registration_request.last_name.clone(),
                    };
                    collection_user.insert_one(&user, None).await?;
                    match create_token_and_refresh(user._id.clone(), JWT_SECRET, REFRESH_JWT_SECRET)
                    {
                        Ok(tokens) => Ok(RegistrationError::Ok(tokens)),
                        Err(_) => Ok(RegistrationError::Unknown),
                    }
                }
                Err(_) => Ok(RegistrationError::WrongPassword),
            },
            Err(_) => Ok(RegistrationError::Unknown),
        }
    }
}
