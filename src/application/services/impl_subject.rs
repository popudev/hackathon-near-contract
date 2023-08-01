use crate::models::contract::{SuperSchoolContract, SuperSchoolContractExt};
use crate::models::subject::{SubjectFeatures, SubjectId, SubjectMetadata};
use near_sdk::{env, near_bindgen, Balance};

#[near_bindgen]
impl SubjectFeatures for SuperSchoolContract {
  fn create_subject(
    &mut self,
    subject_id: SubjectId,
    thumbnail: Option<String>,
    prerequisite_subject_id: Option<SubjectId>,
    title: String,
    description: String,
    price: Balance,
    number_of_credits: u64,
  ) {
    assert!(!self.subject_metadata_by_id.contains_key(&subject_id), "Môn học đã tồn tại");

    let subject_metadata = SubjectMetadata {
      subject_id: subject_id.clone(),
      title,
      thumbnail,
      description,
      number_of_credits,
      price,
      number_students_studing: 0,
      instructor_id: None,
      prerequisite_subject_id,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    self.subject_ids.insert(&subject_id);

    self.subject_metadata_by_id.insert(&subject_id, &subject_metadata);
  }

  fn update_subject(
    &mut self,
    subject_id: SubjectId,
    thumbnail: Option<String>,
    prerequisite_subject_id: Option<SubjectId>,
    title: Option<String>,
    description: Option<String>,
    price: Option<Balance>,
    number_of_credits: Option<u64>,
  ) {
    // let signer_account_id = env::signer_account_id();
    // assert!(self.owner_id == signer_account_id, "Bạn không có quyền chỉnh sửa thông tin");
    assert!(self.subject_metadata_by_id.contains_key(&subject_id), "Môn học không tồn tại");

    let mut subject = self.subject_metadata_by_id.get(&subject_id).unwrap();

    subject.thumbnail = thumbnail;

    subject.prerequisite_subject_id = prerequisite_subject_id;

    if let Some(title) = title {
      subject.title = title;
    }

    if let Some(description) = description {
      subject.description = description;
    }

    if let Some(price) = price {
      subject.price = price;
    }

    if let Some(number_of_credits) = number_of_credits {
      subject.number_of_credits = number_of_credits;
    }

    self.subject_metadata_by_id.insert(&subject_id, &subject);
  }

  fn get_all_subject_metadata(&self) -> Vec<SubjectMetadata> {
    let mut all_subject = Vec::new();

    for subject_id in self.subject_ids.iter() {
      all_subject.push(self.subject_metadata_by_id.get(&subject_id).unwrap());
    }

    all_subject
  }
}
