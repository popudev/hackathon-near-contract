use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

pub type MajorId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MajorMetadata {
  pub major_id: MajorId,
  pub thumbnail: Option<String>,
  pub name: String,
  pub description: String,
  pub number_of_credits_required: u64,
  pub number_students_register: u64,
  pub number_students_studing: u64,
  pub created_at: u64,
  pub updated_at: u64,
}

pub trait MajorFeatures {
  fn create_major(
    &mut self,
    major_id: String,
    thumbnail: Option<String>,
    name: String,
    description: String,
    number_of_credits_required: u64,
  );

  fn update_major(
    &mut self,
    major_id: String,
    thumbnail: Option<String>,
    name: Option<String>,
    description: Option<String>,
    number_of_credits_required: Option<u64>,
  );

  fn get_all_major_metadata(&self) -> Vec<MajorMetadata>;
}
