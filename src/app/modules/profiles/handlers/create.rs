use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::profiles::model::{NewProfile, PostProfile, Profile};

use crate::app::modules::profiles::services::repository as profile_repository;

pub async fn post_create_admin(
    db: &Db,
    _user: UserInClaims,
    profile: PostProfile,
) -> Result<Json<Profile>, Status> {
    let uuid = rocket::serde::uuid::Uuid::new_v4();
    let new_profile = NewProfile {
        name: profile.name,
        surname: profile.surname,
        email: profile.email,
        profile_token: uuid.to_string(),
        user_id: profile.user_id,
    };

    let profile = match profile_repository::create(db, new_profile).await {
        Ok(profile) => profile,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(profile))
}
