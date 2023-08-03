use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, AccountId};

use crate::models::{
  contract::{ContractStorageKey, SuperSchoolContract},
  subject::SubjectId,
  user::UserId,
};

use super::hash_string;

impl SuperSchoolContract {
  pub(crate) fn internal_add_subject_to_user(&mut self, subject_id: &SubjectId, user_id: &UserId) {
    let subject_user_id = format!("{}{}", &subject_id, &user_id);
    let mut set = self.subjects_per_user.get(&user_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ContractStorageKey::SubjectsPerUserInter {
          subject_student_id_hash: hash_string(&subject_user_id),
        }
        .try_to_vec()
        .unwrap(),
      )
    });

    set.insert(subject_id);

    self.subjects_per_user.insert(user_id, &set);
  }
}
