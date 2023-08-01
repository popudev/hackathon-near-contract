use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Balance;

use super::user::UserId;

pub type SubjectId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SubjectMetadata {
  pub subject_id: SubjectId,
  pub instructor_id: Option<UserId>,
  pub prerequisite_subject_id: Option<SubjectId>,
  pub thumbnail: Option<String>,
  pub title: String,
  pub description: String,
  pub number_of_credits: u64,
  pub price: Balance,
  pub students_studying_map: HashMap<UserId, u64>,
  pub students_completed: HashMap<UserId, u64>,
  pub created_at: u64,
  pub updated_at: u64,
}

pub trait SubjectFeatures {
  fn create_subject(
    &mut self,
    subject_id: String,
    thumnail: Option<String>,
    title: String,
    description: String,
    number_of_credits: u64,
  ) -> SubjectMetadata;

  fn get_all_major_metadata(&self);
}
