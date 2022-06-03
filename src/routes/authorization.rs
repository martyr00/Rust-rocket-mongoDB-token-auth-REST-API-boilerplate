use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::database::connect_to_db::MongoDB;
use crate::get_valid_text;
use crate::helper::hash_text;
use crate::routes::routes_models::registration_request::RegistrationRequest;

#[post("/registration", format = "json", data = "<maybe_registration_request>")]
pub async fn registration(
    database: &State<MongoDB>,
    maybe_registration_request: Option<Json<RegistrationRequest>>
) -> Result<Status, Status> {
    match maybe_registration_request {
        None => { Err(Status::BadRequest) },
        Some(registration_request) => {
            match valid_password_and_login(&registration_request.login, &registration_request.password) {
                GetIsValidLoginAndPassword::Ok => {
                    match database.check_login_in_db(registration_request.login.clone()).await {
                        Ok(Some(_)) => { Err(Status::BadRequest) }, //login busy
                        Ok(None) => {
                            match database.registration(registration_request).await {
                                Ok(true) => { Ok(Status::Ok) }, //todo response TOKEN
                                Ok(false) => { Err(Status::BadRequest) }, // bad password
                                Err(_) => { Err(Status::BadRequest) } // bad password
                            }
                        },
                        Err(_) => { Err(Status::InternalServerError) }, //other
                    }
                },
                GetIsValidLoginAndPassword::BadLogin => {
                    Err(Status::BadRequest) // bad login
                },
                GetIsValidLoginAndPassword::BadPassword => {
                    Err(Status::BadRequest) //bad password
                }
            }
        }
    }
}

enum GetIsValidLoginAndPassword {
    Ok,
    BadLogin,
    BadPassword
}

fn valid_password_and_login(login: &str, password: &str) -> GetIsValidLoginAndPassword {
    if get_valid_text(login, 200, 2) {
        if get_valid_text(password, 200, 2) {
            GetIsValidLoginAndPassword::Ok
        } else { GetIsValidLoginAndPassword::BadPassword }
    } else { GetIsValidLoginAndPassword::BadLogin }

}