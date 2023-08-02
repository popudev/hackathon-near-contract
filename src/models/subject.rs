use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{Balance, Promise};

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
  pub number_students_studing: u64,
  pub created_at: u64,
  pub updated_at: u64,
}

pub trait SubjectFeatures {
  fn create_subject(
    &mut self,
    subject_id: SubjectId,
    thumbnail: Option<String>,
    prerequisite_subject_id: Option<SubjectId>,
    title: String,
    description: String,
    price: Balance,
    number_of_credits: u64,
  );

  fn update_subject(
    &mut self,
    subject_id: SubjectId,
    thumbnail: Option<String>,
    prerequisite_subject_id: Option<SubjectId>,
    title: Option<String>,
    description: Option<String>,
    price: Option<Balance>,
    number_of_credits: Option<u64>,
  );

  fn register_subject(&mut self, subject_id: SubjectId);

  fn get_all_subject_metadata(&self) -> Vec<SubjectMetadata>;
}
