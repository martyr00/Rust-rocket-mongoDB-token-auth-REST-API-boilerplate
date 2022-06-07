// use std::str::SplitWhitespace;
// use jsonwebtoken::errors::ErrorKind::Json;
// use rocket::http::{HeaderMap, Status};
// use rocket::http::hyper::header::AUTHORIZATION;
// use rocket::http::hyper::HeaderValue;
// use rocket::response::status::BadRequest;
// use rocket::State;
// use crate::database::connect_to_db::MongoDB;
//
// #[get("/hello")]
// pub async fn hello_name_user(headers: HeaderMap<'_>) -> Result<String, Status> {
//     match jwt_from_header(&headers) {
//         BearerError::Ok(value) => { Ok(value) }
//         BearerError::Unauthorized => { Err(Status::Unauthorized) }
//     }
// }
//
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
