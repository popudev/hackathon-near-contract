use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, AccountId};

use crate::models::{
  contract::{ContractStorageKey, SuperSchoolContract},
  subject::SubjectId,
  user::UserId,
};

use super::hash_string;

impl SuperSchoolContract {
  pub(crate) fn internal_add_student_to_subject(
    &mut self,
    student_id: &UserId,
    subject_id: &SubjectId,
  ) {
    let student_subject_id = format!("{}{}", &student_id, &subject_id);
    let mut set = self.students_per_subject.get(&subject_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ContractStorageKey::StudentsPerSubjectInter {
          student_subject_id_hash: hash_string(&student_subject_id),
        }
        .try_to_vec()
        .unwrap(),
      )
    });

    set.insert(student_id);

    self.students_per_subject.insert(subject_id, &set);
  }
}
