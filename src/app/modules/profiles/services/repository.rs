#[cfg(feature = "db_diesel")]
use diesel::prelude::*;

#[cfg(feature = "db_diesel")]
use crate::database::schema::profiles;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;
#[cfg(feature = "db_sqlx")]
use sqlx::QueryBuilder;

use crate::app::modules::profiles::model::{NewProfile, Profile};
use crate::database::connection::Db;

pub async fn _get_profile_by_id(db: &Db, id: i32) -> Result<Profile, sqlx::Error> {
    let profile = sqlx::query_as!(
        Profile,
        "SELECT * FROM profiles WHERE id = $1",
        id
    ).fetch_one(&db.0).await?;

    Ok(profile)
}

pub async fn get_profile_by_token(
    db: &Db,
    token: String,
) -> Result<Profile, sqlx::Error> {
    let profile = sqlx::query_as!(
        Profile,
        "SELECT * FROM profiles WHERE profile_token = $1",
        token
    ).fetch_one(&db.0).await?;

    Ok(profile)
}

pub async fn create(db: &Db, profile: NewProfile) -> Result<Profile, sqlx::Error> {
    let profile = sqlx::query_as!(
        Profile,
        "INSERT INTO profiles (user_id, profile_token, name, surname, email) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        profile.user_id,
        profile.profile_token,
        profile.name,
        profile.surname,
        profile.email
    ).fetch_one(&db.0).await?;

    Ok(profile)
}
