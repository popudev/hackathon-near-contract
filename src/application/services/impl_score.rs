use crate::application::repository::convert_to_score_id;
use crate::models::contract::{SuperSchoolContract, SuperSchoolContractExt};
use crate::models::score::{ScoreFeatures, ScoreMetadata};
use crate::models::subject::SubjectId;
use crate::models::user::{UserFeatures, UserId};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
impl ScoreFeatures for SuperSchoolContract {
  fn create_score(&mut self, subject_id: SubjectId, student_id: UserId, score: u64) {
    let signer_account_id = env::signer_account_id();
    assert!(self.user_metadata_by_id.contains_key(&student_id), "Sinh viên không tồn tại");
    assert!(self.user_metadata_by_id.contains_key(&signer_account_id), "Giảng viên không tồn tại");
    assert!(self.subject_metadata_by_id.contains_key(&subject_id), "Môn học không tồn tại");

    let mut student = self.user_metadata_by_id.get(&student_id).unwrap();
    assert!(student.active == true, "Sinh viên chưa được duyệt");

    let instructor = self.user_metadata_by_id.get(&signer_account_id).unwrap();
    assert!(instructor.active == true, "Giảng viên chưa được duyệt");

    let subject = self.subject_metadata_by_id.get(&subject_id).unwrap();

    let score_metadata = ScoreMetadata {
      score_id: convert_to_score_id(&subject_id, &student_id),
      subject_id,
      student_id,
      instructor_id: signer_account_id,
      score,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    if score >= 5 {
      student.subject_ids_studied.push(score_metadata.subject_id.clone());
      student.total_credit += subject.number_of_credits;
      self.update_user_metadate(&student);
    }

    self.scores_metadata_by_id.insert(&score_metadata.score_id, &score_metadata);
    self.internal_add_score_to_user(&score_metadata.student_id, &score_metadata.score_id);
    self.internal_add_score_to_subject(&score_metadata.score_id, &score_metadata.subject_id);
  }

  fn update_score(&mut self, subject_id: SubjectId, student_id: UserId, score: u64) {
    let signer_account_id = env::signer_account_id();
    assert!(self.user_metadata_by_id.contains_key(&student_id), "Sinh viên không tồn tại");
    assert!(self.user_metadata_by_id.contains_key(&signer_account_id), "Giảng viên không tồn tại");

    let mut student = self.user_metadata_by_id.get(&student_id).unwrap();
    assert!(student.active == true, "Sinh viên chưa được duyệt");

    let instructor = self.user_metadata_by_id.get(&signer_account_id).unwrap();
    assert!(instructor.active == true, "Giảng viên chưa được duyệt");

    let score_metadata = ScoreMetadata {
      score_id: convert_to_score_id(&subject_id, &student_id),
      subject_id,
      student_id,
      instructor_id: signer_account_id,
      score,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    if score >= 5 {
      student.subject_ids_studied.push(score_metadata.subject_id.clone());
      self.update_user_metadate(&student);
    }

    self.scores_metadata_by_id.insert(&score_metadata.score_id, &score_metadata);
    self.internal_add_score_to_user(&score_metadata.student_id, &score_metadata.score_id);
  }

  fn get_all_score_metadata_by_user_id(&self, user_id: UserId) -> Vec<ScoreMetadata> {
    let mut all_score = Vec::new();

    if let Some(score_ids) = self.scores_per_user.get(&user_id) {
      for score_id in score_ids.iter() {
        all_score.push(self.scores_metadata_by_id.get(&score_id).unwrap());
      }
    }

    all_score
  }

  fn get_all_score_metadata_by_subject_id(&self, subject_id: SubjectId) -> Vec<ScoreMetadata> {
    let mut all_score = Vec::new();

    if let Some(score_ids) = self.scores_per_subject.get(&subject_id) {
      for score_id in score_ids.iter() {
        all_score.push(self.scores_metadata_by_id.get(&score_id).unwrap());
      }
    }

    all_score
  }
}
