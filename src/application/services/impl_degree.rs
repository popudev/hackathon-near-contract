use crate::application::repository::convert_to_degree_id;
use crate::models::contract::{SuperSchoolContract, SuperSchoolContractExt};
use crate::models::degree::{DegreeFeatures, DegreeMetadata, DegreeType};
use crate::models::user::Roles;
use near_sdk::{env, near_bindgen, Promise, ONE_NEAR};

#[near_bindgen]
impl DegreeFeatures for SuperSchoolContract {
  #[payable]
  fn receive_degree(&mut self) -> Promise {
    let user_id = env::signer_account_id();
    let deposit = env::account_balance();
    assert!(deposit >= 10 * ONE_NEAR, "Bạn chuyển không đủ NEAR");
    assert!(self.user_metadata_by_id.contains_key(&user_id), "Người dùng không tồn tại");

    let student = self.user_metadata_by_id.get(&user_id).unwrap();

    assert!(student.role == Roles::Student, "Người dùng không phải là sinh viên");

    let major_id = student.major_id.unwrap();
    assert!(self.major_metadata_by_id.contains_key(&major_id), "Ngành học không tồn tại");

    let major = self.major_metadata_by_id.get(&major_id).unwrap();

    assert!(
      student.total_credit >= major.number_of_credits_required,
      "Bạn chưa đủ điều kiện để lấy bằng cấp"
    );

    assert!(self.scores_per_user.contains_key(&user_id), "Sinh viên không có bảng điểm");

    let score_ids = self.scores_per_user.get(&user_id).unwrap();
    let mut total_score = 0;

    for score_id in score_ids.iter() {
      let score = self.scores_metadata_by_id.get(&score_id).unwrap();
      total_score += score.score;
    }

    let gpa = (total_score as f64 / score_ids.len() as f64) as u64;
    let mut degree_type = DegreeType::Average;

    if gpa >= 7 {
      degree_type = DegreeType::AboveAverage;
    } else if gpa >= 8 {
      degree_type = DegreeType::Good;
    } else if gpa >= 9 {
      degree_type = DegreeType::Excellent;
    }

    let degree = DegreeMetadata {
      degree_id: convert_to_degree_id(&user_id, &major.major_id),
      school_name: self.metadata_contract.get().unwrap().name,
      degree_type,
      gpa,
      thumbnail: None,
      major_id: major.major_id,
      student_id: user_id.clone(),
      created_at: env::block_timestamp(),
    };

    self.degree_metadata_by_id.insert(&user_id, &degree);

    Promise::new(self.owner_id.clone()).transfer(10 * ONE_NEAR)
  }

  fn get_degree_metadata(&self) -> DegreeMetadata {
    let user_id = env::signer_account_id();
    assert!(self.degree_metadata_by_id.contains_key(&user_id), "Người dùng chưa được cấp băng cấp");

    self.degree_metadata_by_id.get(&user_id).unwrap()
  }
}
