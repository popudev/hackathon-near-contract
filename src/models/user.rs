#![allow(clippy::too_many_arguments)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, Promise};

use super::major::MajorId;
use super::subject::SubjectId;

#[derive(Deserialize, BorshDeserialize, BorshSerialize, Serialize, Default, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Roles {
  #[default]
  Student,
  Instructor,
  Admin,
}

pub type UserId = AccountId;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserMetadata {
  pub user_id: UserId,
  pub username: Option<String>,
  pub password: Option<String>,

  pub avatar: Option<String>,
  pub active: bool,
  pub full_name: String,
  pub date_of_birth: String,
  pub email: String,
  pub phone: String,
  pub national_identity_card: String,
  pub national_identity_card_date: String,

  pub major_id: Option<MajorId>,
  pub subject_ids_studied: Vec<SubjectId>,

  pub role: Roles,
  pub total_credit: u32,
  pub balance: Balance,
  pub created_at: u64,
  pub updated_at: u64,
}

pub trait UserFeatures {
  fn create_admin_user(
    &mut self,
    username: String,
    password: String,
    full_name: String,
    date_of_birth: String,
    email: String,
    phone: String,
    national_identity_card: String,
    national_identity_card_date: String,
  );

  fn create_student_user(
    &mut self,
    user_id: UserId,
    full_name: String,
    date_of_birth: String,
    emai: String,
    phone: String,
    national_identity_card: String,
    national_identity_card_date: String,
  );

  fn create_instructor_user(
    &mut self,
    user_id: UserId,
    full_name: String,
    date_of_birth: String,
    emai: String,
    phone: String,
    national_identity_card: String,
    national_identity_card_date: String,
  );

  fn update_user(
    &mut self,
    user_id: UserId,
    full_name: Option<String>,
    date_of_birth: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    national_identity_card: Option<String>,
    national_identity_card_date: Option<String>,
  );

  fn update_user_metadate(&mut self, user_metadate: &UserMetadata);

  fn register_student_user(&mut self, major_id: String) -> Promise;

  fn register_instructor_user(&mut self) -> Promise;

  fn get_all_user_metadata(&self) -> Vec<UserMetadata>;

  fn active_student_user(&mut self, user_id: UserId, username: String, password: String);

  fn active_instructor_user(&mut self, user_id: UserId, username: String, password: String);

  fn get_user_metadata_by_id(&self, id: UserId) -> Option<UserMetadata>;

  fn get_user_metadata_by_username(&self, username: String) -> Option<UserMetadata>;

  fn get_all_student_user_metadata_by_subject_id(&self, subject_id: SubjectId)
    -> Vec<UserMetadata>;

  fn clean(&mut self);
}
