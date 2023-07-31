use crate::models::contract::{SuperSchoolContract, SuperSchoolContractExt};
use crate::models::major::{MajorFeatures, MajorMetadata};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
impl MajorFeatures for SuperSchoolContract {
  fn create_major(
    &mut self,
    major_id: String,
    thumbnail: Option<String>,
    name: String,
    description: String,
    number_of_credits_required: u64,
  ) -> MajorMetadata {
    assert!(!self.major_metadata_by_id.contains_key(&major_id), "Ngành học đã tồn tại");

    let major_metadata = MajorMetadata {
      major_id: major_id.clone(),
      name,
      thumbnail,
      description,
      number_of_credits_required,
      number_students: 0,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    self.major_ids.insert(&major_id);

    self.major_metadata_by_id.insert(&major_id, &major_metadata);

    major_metadata
  }

  fn get_all_major_metadata(&self) -> Vec<MajorMetadata> {
    let mut all_major = Vec::new();

    for major_id in self.major_ids.iter() {
      all_major.push(self.major_metadata_by_id.get(&major_id).unwrap());
    }

    all_major
  }
}
