use crate::database::connect_to_db::MongoDB;
use crate::helper::hash_text;
use crate::models::model_user::User;
use crate::routes::routes_models::login_request::LoginRequest;
use crate::routes::routes_models::registration_request::RegistrationRequest;
use crate::Status;
use bcrypt::{verify, BcryptResult};
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Database};
use rocket::serde::json::Json;

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

    pub async fn login(&self, login_request: Json<LoginRequest>) -> mongodb::error::Result<bool> {
        match Self::find_user_by_login(self, login_request.login.clone()).await {
            Ok(option_user) => match option_user {
                None => {
                    println!("Wrong login");
                    Ok(false)
                }
                Some(user) => match verify(&login_request.password, &user.password) {
                    Ok(true) => Ok(true),
                    Ok(false) => Ok(false),
                    Err(_) => Ok(false),
                },
            },
            Err(error) => {
                println!("error find_user_by_login: {:?}", error);
                Ok(false)
            }
        }
    }

    pub async fn registration(
        &self,
        registration_request: Json<RegistrationRequest>,
    ) -> mongodb::error::Result<bool> {
        let collection_user = self.database.collection::<User>("user");

        match hash_text(registration_request.password.clone(), 4) {
            Ok(hash_password) => {
                collection_user
                    .insert_one(
                        User {
                            _id: ObjectId::new(),
                            login: registration_request.login.clone(),
                            password: hash_password,
                        },
                        None,
                    )
                    .await?;
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
}
