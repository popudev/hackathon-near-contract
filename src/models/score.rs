use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use super::subject::SubjectId;
use super::user::UserId;

pub type ScoreId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ScoreMetadata {
  pub score_id: ScoreId,
  pub subject_id: SubjectId,
  pub student_id: UserId,
  pub instructor_id: UserId,
  pub score: u64,
  pub created_at: u64,
  pub updated_at: u64,
}

pub trait ScoreFeatures {
  fn create_score(
    &mut self,
    instructor_id: UserId,
    student_id: UserId,
    subject_id: SubjectId,
    score: u64,
  );

  fn update_score(
    &mut self,
    instructor_id: UserId,
    student_id: UserId,
    subject_id: SubjectId,
    score: u64,
  );

  fn get_all_score_metadata_by_user_id(&self, user_id: UserId) -> Vec<ScoreMetadata>;

  fn get_all_score_metadata_by_subject_id(&self, subject_id: SubjectId) -> Vec<ScoreMetadata>;
}
