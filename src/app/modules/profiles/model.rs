use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "db_diesel")]
use crate::database::schema::profiles;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;
#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

// #[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
// #[serde(crate = "rocket::serde")]

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub profile_token: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(feature = "db_diesel", derive(Insertable))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = profiles))]
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewProfile {
    pub user_id: i32,
    pub profile_token: String,
    pub name: String,
    pub surname: String,
    pub email: String,
}

#[cfg_attr(feature = "db_diesel", derive(Insertable))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = profiles))]
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostProfile {
    pub user_id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
}
