use crate::database::connect_to_db::MongoDB;
use crate::database::FindUserBy;
use bcrypt::hash;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;

pub fn get_valid_text(text: &str, max_size: usize, min_size: usize) -> bool {
    return if !text.is_empty() && text.len() <= max_size && text.len() >= min_size {
        true
    } else {
        false
    };
}

pub fn get_valid_name(text: &str, max_size: usize, min_size: usize) -> bool {
    return if text.is_empty() || text.len() <= max_size && text.len() >= min_size {
        true
    } else {
        false
    };
}

pub fn hash_text(text: String, cost: u32) -> Result<String, Status> {
    return match hash(text, cost) {
        Ok(hash_text) => Ok(hash_text),
        Err(_) => Err(Status::BadRequest),
    };
}

pub fn object_id_parse_str(id_str: String) -> Result<ObjectId, String> {
    match ObjectId::parse_str(id_str) {
        Ok(to_id) => Ok(to_id),
        Err(error) => Err(format!("{}", error)),
    }
}

pub async fn find_user_by_login_and_mail(
    database: &MongoDB,
    mail: &str,
    login: &str,
) -> FindUserBy {
    match database.find_user_by("login", login).await {
        Ok(None) => match database.find_user_by("mail", mail).await {
            Ok(None) => FindUserBy::UserNotFound,
            Ok(Some(_)) => FindUserBy::UserFoundByEmail,
            Err(_) => FindUserBy::UserFoundByEmail,
        },
        Ok(Some(_)) => FindUserBy::UserFoundByLogin,
        Err(_) => FindUserBy::UserFoundByLogin,
    }
}
