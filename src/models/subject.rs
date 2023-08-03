use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Balance;

use super::major::MajorId;
use super::user::UserId;

pub type SubjectId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SubjectMetadata {
  pub subject_id: SubjectId,
  pub major_id: MajorId,
  pub instructor_id: Option<UserId>,
  pub prerequisite_subject_id: Option<SubjectId>,
  pub thumbnail: Option<String>,
  pub title: String,
  pub description: String,
  pub number_of_credits: u64,
  pub price: Balance,
  pub balance: Option<Balance>,
  pub number_students_studing: u64,
  pub created_at: u64,
  pub updated_at: u64,
}

pub trait SubjectFeatures {
  fn create_subject(
    &mut self,
    subject_id: SubjectId,
    major_id: MajorId,
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
    major_id: Option<MajorId>,
    thumbnail: Option<String>,
    prerequisite_subject_id: Option<SubjectId>,
    title: Option<String>,
    description: Option<String>,
    price: Option<Balance>,
    number_of_credits: Option<u64>,
  );

  fn register_subject(&mut self, subject_id: SubjectId);

  fn assignment(&mut self, instructor_id: UserId, subject_id: SubjectId);

  fn get_all_subject_metadata(&self) -> Vec<SubjectMetadata>;

  fn get_all_subject_metadata_by_major_id(&self, major_id: MajorId) -> Vec<SubjectMetadata>;

  fn get_all_subject_metadata_by_user_id(&self, user_id: UserId) -> Vec<SubjectMetadata>;
}
