use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, AccountId};

use crate::models::{
  contract::{ContractStorageKey, SuperSchoolContract},
  score::ScoreId,
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
}
