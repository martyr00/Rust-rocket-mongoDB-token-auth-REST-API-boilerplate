use crate::constants::WRONG_REQUEST;
use crate::database::connect_to_db::MongoDB;
use crate::helper::object_id_parse_str;
use crate::{ErrorResponse, Status, UNKNOWN};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::State;
use std::future::Future;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloNameResponse {
    greetings: String,
}

//todo AUTHORIZE from headers!!!
#[get("/hello/<id>")]
pub async fn hello_name_user(
    database: &State<MongoDB>,
    id: String,
) -> Result<Json<HelloNameResponse>, (Status, Json<ErrorResponse>)> {
    match hello_real_names(database, id).await {
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

enum HelloNameError {
    OnlyLogin(String),
    NoOnlyLogin(String),
    ErrorID,
    ErrorUnknown,
}

async fn hello_real_names(database: &State<MongoDB>, id_str: String) -> HelloNameError {
    match object_id_parse_str(id_str) {
        Ok(id) => match database.find_user_by_id(id).await {
            Ok(Some(user)) => {
                if user.first_name == "" {
                    HelloNameError::OnlyLogin(format!("Hello {} {}", user.login, user.last_name))
                } else {
                    HelloNameError::NoOnlyLogin(format!(
                        "Hello {} {} {}",
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

// enum BearerError {
//     Ok(String),
//     Unauthorized
// }
//
// pub struct BasicAuth {
//     pub(crate) username: String,
//     pub(crate) password: String
// }
//
// // fn bearer(header: Option<String>) -> BearerError {
// //     match header {
// //         Some(authorization) => {
// //             let array_header_val = authorization.split_whitespace().collect::<Vec<_>>();
// //             if array_header_val.len() != 2 && array_header_val[1].is_empty() && array_header_val[0] != "Basic" {
// //                 BearerError::Unauthorized
// //             } else {
// //                 BearerError::Ok
// //             }
// //         },
// //         None => { BearerError::Unauthorized },
// //     }
//
//
// // Bearer
// // match get one header Authorization -> (Bearer 'TOKEN')
// //     Some(header) => {
// //         let array_header_val = header.split(" ")
// //             if array_header_val[1].is_empty {return Err(Status::401)}
// //             else {
// //                 array_header_val[1].parse_from_JWD() => struct { user_id: 'ObjectId' }
// //                 if find user in DB by user_id {
// //                     return Ok(Status::Ok)
// //                 } else {
// //                      return Err(Status::401)
// //                 }
// //             }
// //         },
// //     None(_) => return Err(Status::401)
