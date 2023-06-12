use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::constants::ROBOT_TOKEN_EXPIRATION;
use crate::app::providers::guards::claims::AccessClaims;

use crate::app::modules::profiles::handlers::create;
use crate::app::modules::profiles::model::{PostProfile, Profile};
use crate::app::modules::profiles::services::repository as profile_repository;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_token,
        get_index,
        get_index_none,
        get_token,
        get_token_none,
        post_create,
        post_create_none,
    ]
}

#[options("/")]
fn options_index() -> Status {
    Status::Ok
}

#[options("/token")]
fn options_token() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
fn get_index(_access_claims: AccessClaims) -> &'static str {
    "Hello from profiles"
}

#[get("/", rank = 2)]
fn get_index_none() -> Status {
    Status::Unauthorized
}

#[post("/token", data = "<token>", rank = 1)]
async fn get_token(
    db: Db,
    access_claims: AccessClaims,
    token: Json<String>,
) -> Result<Json<i32>, Status> {
    let limit_exp = chrono::Utc::now().timestamp() + ROBOT_TOKEN_EXPIRATION;
    if access_claims.0.exp > limit_exp {
        return Err(Status::Unauthorized);
    }

    // clean token
    let token = token
        .clone()
        .into_inner()
        .replace("\"", "")
        .replace("{ ", "")
        .replace("}", "");
    let token = token.trim_matches('"').trim();

    let profile = profile_repository::get_profile_by_token(&db, token.to_string()).await;
    match profile {
        Ok(profile) => Ok(Json(profile.user_id)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/token", data = "<_token>", rank = 2)]
async fn get_token_none(_token: Json<String>) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<post_profile>", rank = 1)]
async fn post_create(
    db: Db,
    claims: AccessClaims,
    post_profile: Json<PostProfile>,
) -> Result<Json<Profile>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => {
            create::post_create_admin(&db, claims.0.user, post_profile.into_inner()).await
        },
        "robot" => {
            create::post_create_admin(&db, claims.0.user, post_profile.into_inner()).await
        },
        _ => {
            println!(
                "Error: post_create; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[post("/", data = "<_post_profile>", rank = 2)]
async fn post_create_none(_post_profile: Json<PostProfile>) -> Status {
    Status::Unauthorized
}
