use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, AccountId};

use crate::models::{
  contract::{ContractStorageKey, SuperSchoolContract},
  major::MajorId,
  subject::SubjectId,
  user::UserId,
};

use super::hash_string;

impl SuperSchoolContract {
  pub(crate) fn internal_add_student_to_major(&mut self, major_id: &String, user_id: &AccountId) {
    let major_student_id = format!("{}{}", &major_id, &user_id);
    let mut users_set = self.students_per_major.get(&major_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ContractStorageKey::StudentsPerMajorInter {
          major_student_id_hash: hash_string(&major_student_id),
        }
        .try_to_vec()
        .unwrap(),
      )
    });

    users_set.insert(user_id);

    self.students_per_major.insert(major_id, &users_set);
  }

  pub(crate) fn internal_add_subject_to_major(
    &mut self,
    subject_id: &SubjectId,
    major_id: &MajorId,
  ) {
    let subject_major_id_hash = hash_string(&format!("{}{}", &subject_id, &major_id));
    let mut set = self.subjects_per_major.get(&major_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ContractStorageKey::SubjectsPerMajorInter { subject_major_id_hash }.try_to_vec().unwrap(),
      )
    });

    set.insert(subject_id);

    self.subjects_per_major.insert(major_id, &set);
  }
}
