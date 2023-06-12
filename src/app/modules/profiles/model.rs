use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::database::schema::profiles;

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub profile_token: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub user_id: i32,
    pub profile_token: String,
    pub name: String,
    pub surname: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = profiles)]
pub struct PostProfile {
    pub user_id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
}
