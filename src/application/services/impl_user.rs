use crate::models::contract::{SuperSchoolContract, SuperSchoolContractExt};
use crate::models::subject::SubjectId;
use crate::models::user::{Roles, UserFeatures, UserId, UserMetadata};
use near_sdk::{env, near_bindgen, Promise, ONE_NEAR};

#[near_bindgen]
impl UserFeatures for SuperSchoolContract {
  fn create_admin_user(&mut self, username: String, password: String) {
    let signer_account_id = env::signer_account_id();
    assert!(self.owner_id == signer_account_id, "Bạn không có quyền tạo quản trị viên");

    let user_metadata = UserMetadata {
      user_id: signer_account_id.clone(),
      username: Some(username.clone()),
      password: Some(password),
      full_name: "Admin".to_string(),
      date_of_birth: "1/1/1111".to_string(),
      email: "admin@admin".to_string(),
      phone: "0999099099".to_string(),
      national_identity_card: "123456".to_string(),
      national_identity_card_date: "12/34".to_string(),
      active: true,
      total_credit: 99999,
      avatar: None,
      major_id: None,
      balance: 9999,
      role: Roles::Admin,
      subject_ids_studied: Vec::new(),
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    self.user_ids.insert(&signer_account_id);

    self.user_metadata_by_id.insert(&signer_account_id, &user_metadata);
    self.user_metadata_by_username.insert(&username, &user_metadata);
  }

  fn create_student_user(
    &mut self,
    user_id: UserId,
    full_name: String,
    date_of_birth: String,
    email: String,
    phone: String,
    national_identity_card: String,
    national_identity_card_date: String,
  ) {
    // let signer_account_id = env::signer_account_id();
    // assert!(self.owner_id == signer_account_id, "Bạn không có quyền thêm người dùng");
    assert!(!self.user_metadata_by_id.contains_key(&user_id), "User has already exists");

    let user_metadata = UserMetadata {
      user_id: user_id.clone(),
      username: None,
      password: None,
      full_name,
      date_of_birth,
      email,
      phone,
      national_identity_card,
      national_identity_card_date,
      active: false,
      total_credit: 0,
      avatar: None,
      major_id: None,
      subject_ids_studied: Vec::new(),
      balance: 0,
      role: Roles::Student,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    self.user_ids.insert(&user_id);

    self.user_metadata_by_id.insert(&user_id, &user_metadata);
  }

  fn create_instructor_user(
    &mut self,
    user_id: UserId,
    full_name: String,
    date_of_birth: String,
    email: String,
    phone: String,
    national_identity_card: String,
    national_identity_card_date: String,
  ) {
    // let signer_account_id = env::signer_account_id();
    // assert!(self.owner_id == signer_account_id, "Bạn không có quyền thêm người dùng");
    assert!(!self.user_metadata_by_id.contains_key(&user_id), "User has already exists");

    let user_metadata = UserMetadata {
      user_id: user_id.clone(),
      username: None,
      password: None,
      full_name,
      date_of_birth,
      email,
      phone,
      national_identity_card,
      national_identity_card_date,
      active: false,
      total_credit: 0,
      avatar: None,
      balance: 0,
      major_id: None,
      subject_ids_studied: Vec::new(),
      role: Roles::Instructor,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    self.user_ids.insert(&user_id);

    self.user_metadata_by_id.insert(&user_id, &user_metadata);
  }

  fn update_user(
    &mut self,
    user_id: UserId,
    full_name: Option<String>,
    date_of_birth: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    national_identity_card: Option<String>,
    national_identity_card_date: Option<String>,
  ) {
    // let signer_account_id = env::signer_account_id();
    // assert!(self.owner_id == signer_account_id, "Bạn không có quyền chỉnh sửa thông tin");
    assert!(self.user_metadata_by_id.contains_key(&user_id), "Người dùng không tồn tại");

    let mut user = self.user_metadata_by_id.get(&user_id).unwrap();

    if let Some(full_name) = full_name {
      user.full_name = full_name;
    }

    if let Some(date_of_birth) = date_of_birth {
      user.date_of_birth = date_of_birth;
    }

    if let Some(email) = email {
      user.email = email;
    }

    if let Some(phone) = phone {
      user.phone = phone;
    }

    if let Some(national_identity_card) = national_identity_card {
      user.national_identity_card = national_identity_card;
    }

    if let Some(national_identity_card_date) = national_identity_card_date {
      user.national_identity_card_date = national_identity_card_date;
    }

    self.update_user_metadate(&user);
  }

  #[private]
  fn update_user_metadate(&mut self, user_metadate: &UserMetadata) {
    self.user_metadata_by_id.insert(&user_metadate.user_id, &user_metadate);

    if let Some(username) = user_metadate.username.clone() {
      self.user_metadata_by_username.insert(&username, &user_metadate);
    }
  }

  #[payable]
  fn register_student_user(&mut self, major_id: String) -> Promise {
    let user_id = env::signer_account_id();
    assert!(self.user_metadata_by_id.contains_key(&user_id), "Người dùng không tồn tại");
    assert!(self.major_metadata_by_id.contains_key(&major_id), "Ngành học không tồn tại");

    let mut student = self.user_metadata_by_id.get(&user_id).unwrap();
    assert!(student.major_id.is_none(), "Bạn đã đăng ký ngành học");

    student.balance = 5;
    student.major_id = Some(major_id.clone());
    self.update_user_metadate(&student);

    let mut major = self.major_metadata_by_id.get(&major_id).unwrap();
    major.number_students_register += 1;
    self.major_metadata_by_id.insert(&major_id, &major);

    self.internal_add_student_to_major(&major_id, &user_id);

    // Phí đăng ký nhập học
    Promise::new(self.owner_id.clone()).transfer(5 * ONE_NEAR)
  }

  #[payable]
  fn register_instructor_user(&mut self) -> Promise {
    let user_id = env::signer_account_id();
    assert!(self.user_metadata_by_id.contains_key(&user_id), "Người dùng không tồn tại");

    let mut instructor = self.user_metadata_by_id.get(&user_id).unwrap();

    instructor.balance = 10;

    self.update_user_metadate(&instructor);

    // Phí đăng ký làm giảng viên
    Promise::new(self.owner_id.clone()).transfer(10)
  }

  fn active_student_user(&mut self, user_id: UserId, username: String, password: String) {
    let signer_account_id = env::signer_account_id();

    assert!(self.owner_id == signer_account_id, "Bạn không có quyền");
    assert!(self.user_metadata_by_id.contains_key(&user_id), "Người dùng không tồn tại");

    let mut student = self.user_metadata_by_id.get(&user_id).unwrap();

    assert!(student.balance == 5, "Sinh viên chưa nộp phí đăng ký");

    student.active = true;
    student.balance = 0;
    student.username = Some(username.clone());
    student.password = Some(password);

    self.user_metadata_by_id.insert(&user_id, &student);
    self.user_metadata_by_username.insert(&username, &student);
  }

  fn active_instructor_user(&mut self, user_id: UserId, username: String, password: String) {
    let signer_account_id = env::signer_account_id();

    assert!(self.owner_id == signer_account_id, "Bạn không có quyền");
    assert!(self.user_metadata_by_id.contains_key(&user_id), "Người dùng không tồn tại");

    let mut instructor = self.user_metadata_by_id.get(&user_id).unwrap();

    assert!(instructor.balance == 10, "Giảng viên này chưa nộp phí đăng ký");

    instructor.active = true;
    instructor.balance = 0;
    instructor.username = Some(username.clone());
    instructor.password = Some(password);

    self.user_metadata_by_id.insert(&user_id, &instructor);
    self.user_metadata_by_username.insert(&username, &instructor);
  }

  fn get_all_user_metadata(&self) -> Vec<UserMetadata> {
    let mut all_user = Vec::new();

    for user_id in self.user_ids.iter() {
      all_user.push(self.user_metadata_by_id.get(&user_id).unwrap());
    }

    all_user
  }

  fn get_user_metadata_by_id(&self, user_id: UserId) -> Option<UserMetadata> {
    if let Some(metadata) = self.user_metadata_by_id.get(&user_id) {
      Some(metadata)
    } else {
      None
    }
  }

  fn get_user_metadata_by_username(&self, username: String) -> Option<UserMetadata> {
    if let Some(metadata) = self.user_metadata_by_username.get(&username) {
      Some(metadata)
    } else {
      None
    }
  }

  fn get_all_student_user_metadata_by_subject_id(
    &self,
    subject_id: SubjectId,
  ) -> Vec<UserMetadata> {
    let mut all_user = Vec::new();

    if let Some(user_ids) = self.students_per_subject.get(&subject_id) {
      for user_id in user_ids.iter() {
        all_user.push(self.user_metadata_by_id.get(&user_id).unwrap());
      }
    }

    all_user
  }

  fn clean(&mut self) {
    for user_id in self.user_ids.iter() {
      self.user_metadata_by_id.remove(&user_id);
    }
    self.user_ids.clear();
  }
}
