use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Promise;

use super::user::UserId;

pub type DegreeId = String;

#[derive(Deserialize, BorshDeserialize, BorshSerialize, Serialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum DegreeType {
  Average,
  AboveAverage,
  Good,
  Excellent,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DegreeMetadata {
  pub degree_id: String,
  pub degree_type: DegreeType,
  pub gpa: u64,
  pub thumbnail: Option<String>,
  pub school_name: String,
  pub major_id: String,
  pub student_id: UserId,
  pub created_at: u64,
}

pub trait DegreeFeatures {
  fn receive_degree(&mut self) -> Promise;
  fn get_degree_metadata(&self) -> DegreeMetadata;
}
