use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub email_confirmed: bool,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserClue {
    pub id: i32,
    pub user_id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nick_name: String,
    pub phone_number: String,
    pub mobile_number: String,
    pub gender: String,
    pub birth_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBankAccount {
    pub id: i32,
    pub user_id: i32,
    pub account_name: String,
    pub account_no: String,
    pub bank_address: String,
    pub bank_branch: String,
    pub swift_code: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserIdentity {
    pub id: i32,
    pub user_id: i32,
    pub account_email: String,
    pub account_id: String,
    pub provider: String,
    pub public_profile_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserWorkExperience {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub organization: String,
    pub location: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserWorkPreference {
    pub id: i32,
    pub user_id: i32,
    pub interests: i32,
    pub project_limit: i32,
    pub project_limit_updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
