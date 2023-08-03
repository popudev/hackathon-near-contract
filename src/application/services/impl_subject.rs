use crate::models::contract::{SuperSchoolContract, SuperSchoolContractExt};
use crate::models::major::MajorId;
use crate::models::subject::{SubjectFeatures, SubjectId, SubjectMetadata};
use crate::models::user::{Roles, UserId};
use near_sdk::{env, near_bindgen, Balance, Promise, ONE_NEAR};

#[near_bindgen]
impl SubjectFeatures for SuperSchoolContract {
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
  ) {
    let signer_account_id = env::signer_account_id();
    assert!(self.owner_id == signer_account_id, "Bạn không có quyền chỉnh sửa thông tin");
    assert!(self.major_metadata_by_id.contains_key(&major_id), "Ngành học không tồn tại");
    assert!(!self.subject_metadata_by_id.contains_key(&subject_id), "Môn học đã tồn tại");

    let subject_metadata = SubjectMetadata {
      subject_id: subject_id.clone(),
      major_id: major_id.clone(),
      title,
      thumbnail,
      description,
      number_of_credits,
      price: price * ONE_NEAR,
      number_students_studing: 0,
      instructor_id: None,
      prerequisite_subject_id,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };
    self.internal_add_subject_to_major(&subject_id, &major_id);
    self.subject_ids.insert(&subject_id);
    self.subject_metadata_by_id.insert(&subject_id, &subject_metadata);
  }

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
  ) {
    let signer_account_id = env::signer_account_id();
    assert!(self.owner_id == signer_account_id, "Bạn không có quyền chỉnh sửa thông tin");
    assert!(self.subject_metadata_by_id.contains_key(&subject_id), "Môn học không tồn tại");

    let mut subject = self.subject_metadata_by_id.get(&subject_id).unwrap();

    subject.thumbnail = thumbnail;
    subject.prerequisite_subject_id = prerequisite_subject_id;

    if let Some(major_id) = major_id {
      assert!(self.major_metadata_by_id.contains_key(&major_id), "Ngành học không tồn tại");
      subject.major_id = major_id;
    }

    if let Some(title) = title {
      subject.title = title;
    }

    if let Some(description) = description {
      subject.description = description;
    }

    if let Some(price) = price {
      subject.price = price * ONE_NEAR;
    }

    if let Some(number_of_credits) = number_of_credits {
      subject.number_of_credits = number_of_credits;
    }

    self.subject_metadata_by_id.insert(&subject_id, &subject);
  }

  #[payable]
  fn register_subject(&mut self, subject_id: SubjectId) {
    let user_id = env::signer_account_id();
    assert!(self.user_metadata_by_id.contains_key(&user_id), "Người dùng không tồn tại");
    assert!(self.subject_metadata_by_id.contains_key(&subject_id), "Môn học không tồn tại");

    let student = self.user_metadata_by_id.get(&user_id).unwrap();
    assert!(student.active == true, "Bạn chưa được phép đăng ký môn học");
    assert!(student.role == Roles::Student, "Bạn không phải là sinh viên");

    let subject = self.subject_metadata_by_id.get(&subject_id).unwrap();

    if let Some(prerequisite_subject_id) = subject.prerequisite_subject_id {
      assert!(
        student.subject_ids_studied.contains(&prerequisite_subject_id),
        "Yêu cầu học môn tiên quyết của môn học này"
      );
    }

    self.internal_add_subject_to_user(&subject_id, &user_id);
    self.internal_add_student_to_subject(&user_id, &subject_id);

    let salary = (subject.price as f64 * 0.5) as u128;
    Promise::new(self.owner_id.clone()).transfer(subject.price - salary);
    Promise::new(subject.instructor_id.unwrap()).transfer(salary);
  }

  fn assignment(&mut self, instructor_id: UserId, subject_id: SubjectId) {
    let signer_account_id = env::signer_account_id();
    assert!(self.owner_id == signer_account_id, "Bạn không có quyền chỉnh sửa thông tin");
    assert!(self.user_metadata_by_id.contains_key(&instructor_id), "Người dùng không tồn tại");
    assert!(self.subject_metadata_by_id.contains_key(&subject_id), "Khóa học không tồn tại");

    let instructor = self.user_metadata_by_id.get(&instructor_id).unwrap();
    assert!(instructor.role == Roles::Instructor, "Người dùng này không phải giảng viên");
    assert!(instructor.active == true, "Người dùng này chưa được duyệt");

    let mut subject = self.subject_metadata_by_id.get(&subject_id).unwrap();
    self.internal_add_subject_to_user(&subject_id, &instructor_id);
    subject.instructor_id = Some(instructor_id);

    self.subject_metadata_by_id.insert(&subject_id, &subject);
  }

  fn get_all_subject_metadata(&self) -> Vec<SubjectMetadata> {
    let mut all_subject = Vec::new();

    for subject_id in self.subject_ids.iter() {
      all_subject.push(self.subject_metadata_by_id.get(&subject_id).unwrap());
    }

    all_subject
  }

  fn get_all_subject_metadata_by_user_id(&self, user_id: UserId) -> Vec<SubjectMetadata> {
    let mut all_subject = Vec::new();

    if let Some(subject_ids) = self.subjects_per_user.get(&user_id) {
      for subject_id in subject_ids.iter() {
        all_subject.push(self.subject_metadata_by_id.get(&subject_id).unwrap());
      }
    }

    all_subject
  }

  fn get_all_subject_metadata_by_major_id(&self, major_id: MajorId) -> Vec<SubjectMetadata> {
    let mut all_subject = Vec::new();

    if let Some(subject_ids) = self.subjects_per_major.get(&major_id) {
      for subject_id in subject_ids.iter() {
        all_subject.push(self.subject_metadata_by_id.get(&subject_id).unwrap());
      }
    }

    all_subject
  }
}
