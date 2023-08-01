use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, AccountId};

use crate::models::{
  contract::{ContractStorageKey, SuperSchoolContract},
  user::UserId,
};

use super::hash_string;

impl SuperSchoolContract {
  pub(crate) fn internal_add_student_to_major(&mut self, major_id: &String, user_id: &AccountId) {
    let mut users_set = self.students_per_major.get(&major_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ContractStorageKey::SubjectsPerMajorInter { instructor_id_hash: hash_string(major_id) }.try_to_vec().unwrap(),
      )
    });

    //we insert the token ID into the set
    users_set.insert(user_id);

    self.students_per_major.insert(major_id, &users_set);
  }
}
