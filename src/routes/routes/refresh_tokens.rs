use crate::constants::{EXPIRATION_REFRESH_TOKEN, EXPIRATION_TOKEN};
use crate::helper::object_id_parse_str;
use crate::models::tokens::Token;
use crate::private::{JWT_SECRET, REFRESH_JWT_SECRET};
use crate::routes::authorization::token::create_token::encode_token_and_refresh;
use crate::routes::authorization::token::request_refresh_token::RefreshTokensAuthorizedUser;
use crate::{ErrorResponse, Status, UNAUTHORIZED};

use rocket::serde::json::Json;

//refresh_tokens
#[post("/refresh")]
pub async fn refresh_tokens(
    refresh: RefreshTokensAuthorizedUser,
) -> Result<Json<Token>, (Status, Json<ErrorResponse>)> {
    match object_id_parse_str(refresh.user_id) {
        Ok(id) => {
            match encode_token_and_refresh(
                id,
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
        Err(_) => Err(UNAUTHORIZED),
    }
}
