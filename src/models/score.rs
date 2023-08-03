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
  fn create_score(&mut self, subject_id: SubjectId, student_id: UserId, score: u64);

  fn update_score(&mut self, subject_id: SubjectId, student_id: UserId, score: u64);

  // fn get_all_score_metadata_by_user_id(&self) -> Vec<ScoreMetadata>;

  // fn get_all_score_metadata(&self) -> Vec<ScoreMetadata>;
}
