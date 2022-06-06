// use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
// use mongodb::bson::oid::ObjectId;
// use rocket::futures::future::err;
// use serde::{Deserialize, Serialize};
//
// pub enum CreateTokenOutcome {
//     Ok(String),
//     Err,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     user_id: String,
// }
//
// pub fn create_jwt(user_object_id: ObjectId) -> CreateTokenOutcome {
//     match user_object_id {
//         Ok(user_id_string) => {
//             let my_claims = Claims {
//                 user_id: user_id_string,
//             };
//             CreateTokenOutcome::Ok(
//                 encode(
//                     &Header::default(),
//                     &my_claims,
//                     &EncodingKey::from_secret("secret".as_ref()),
//                 )
//                 .unwrap(),
//             )
//         }
//         Err(_) => Err,
//     }
// }
