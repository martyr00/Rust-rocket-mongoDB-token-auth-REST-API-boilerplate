use crate::constants::WRONG_REQUEST;
use crate::database::connect_to_db::MongoDB;
use crate::helper::object_id_parse_str;
use crate::models::hello_response::HelloNameResponse;
use crate::routes::test_routes::HelloNameError;
use crate::{ErrorResponse, Status, UNKNOWN};

use crate::routes::authorization::token::request_access_token::AuthorizedUser;
use rocket::serde::json::Json;
use rocket::State;

//(private) request with authorization model (token)
#[get("/hello")]
pub async fn hello_name_user(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
) -> Result<Json<HelloNameResponse>, (Status, Json<ErrorResponse>)> {
    println!("{}", &auth.user_id);
    match check_from_db_real_names(database, auth.user_id).await {
        HelloNameError::OnlyLogin(res_only_login) => Ok(Json(HelloNameResponse {
            greetings: res_only_login,
        })),
        HelloNameError::NoOnlyLogin(res_no_only_login) => Ok(Json(HelloNameResponse {
            greetings: res_no_only_login,
        })),
        HelloNameError::ErrorID => Err(WRONG_REQUEST),
        HelloNameError::ErrorUnknown => Err(UNKNOWN),
    }
}

//we check if the first and last names are in the database
async fn check_from_db_real_names(database: &State<MongoDB>, id_str: String) -> HelloNameError {
    match object_id_parse_str(id_str) {
        Ok(id) => match database.find_user_by_id(id).await {
            Ok(Some(user)) => {
                if user.first_name == "" || user.last_name == "" {
                    HelloNameError::OnlyLogin(format!("Hello {}", user.login,))
                } else {
                    HelloNameError::NoOnlyLogin(format!(
                        "Hello {} <{}> {}",
                        user.first_name, user.login, user.last_name
                    ))
                }
            }
            Ok(None) => HelloNameError::ErrorID,
            Err(_) => HelloNameError::ErrorUnknown,
        },
        Err(_) => HelloNameError::ErrorID,
    }
}

//(public) hello world
#[get("/public/hello")]
pub async fn hello_world() -> Json<&'static str> {
    Json("Hello world")
}
