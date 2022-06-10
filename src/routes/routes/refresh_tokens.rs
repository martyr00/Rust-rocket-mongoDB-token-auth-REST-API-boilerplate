use crate::constants::{EXPIRATION_REFRESH_TOKEN, EXPIRATION_TOKEN};
use crate::helper::{parse_id_and_find_user_by_id, FindUserById};
use crate::models::tokens::Token;
use crate::private::{JWT_SECRET, REFRESH_JWT_SECRET};
use crate::routes::authorization::token::create_token::encode_token_and_refresh;
use crate::routes::authorization::token::request_refresh_token::RefreshTokensAuthorizedUser;
use crate::{ErrorResponse, Status, UNAUTHORIZED};

use crate::database::connect_to_db::MongoDB;
use rocket::serde::json::Json;
use rocket::State;

//refresh_tokens
#[post("/refresh")]
pub async fn refresh_tokens(
    database: &State<MongoDB>,
    refresh: RefreshTokensAuthorizedUser,
) -> Result<Json<Token>, (Status, Json<ErrorResponse>)> {
    match parse_id_and_find_user_by_id(database, refresh.user_id).await {
        FindUserById::Ok(user) => {
            match encode_token_and_refresh(
                user._id,
                JWT_SECRET,
                REFRESH_JWT_SECRET,
                EXPIRATION_REFRESH_TOKEN,
                EXPIRATION_TOKEN,
            ) {
                Ok(tokens) => Ok(Json(Token {
                    token: tokens.token,
                    refresh_token: tokens.refresh_token,
                })),
                Err(_) => Err(UNAUTHORIZED),
            }
        }
        FindUserById::NoneUser => Err(UNAUTHORIZED),
        FindUserById::BadId => Err(UNAUTHORIZED),
    }
}
