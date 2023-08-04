use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, AccountId};

use crate::models::{
  contract::{ContractStorageKey, SuperSchoolContract},
  score::ScoreId,
  subject::{self, SubjectId},
  user::UserId,
};

use super::hash_string;

impl SuperSchoolContract {
  pub(crate) fn internal_add_score_to_user(&mut self, user_id: &AccountId, score_id: &ScoreId) {
    let score_student_id = format!("{}{}", &score_id, &user_id);
    let mut set = self.scores_per_user.get(&user_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ContractStorageKey::ScoresPerUserInter {
          score_student_id_hash: hash_string(&score_student_id),
        }
        .try_to_vec()
        .unwrap(),
      )
    });

    set.insert(score_id);

    self.scores_per_user.insert(user_id, &set);
  }

  pub(crate) fn internal_add_score_to_subject(
    &mut self,
    score_id: &ScoreId,
    subject_id: &SubjectId,
  ) {
    let score_subject_id_hash = hash_string(&format!("{}{}", &score_id, &subject_id));
    let mut set = self.scores_per_subject.get(&subject_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ContractStorageKey::ScoresPerSubjectInter { score_subject_id_hash }.try_to_vec().unwrap(),
      )
    });

    set.insert(score_id);

    self.scores_per_subject.insert(subject_id, &set);
  }
}
